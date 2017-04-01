#[macro_use] extern crate glium;
extern crate osu_format;
extern crate rodio;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

#[derive(Debug, Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}
implement_vertex!(Vertex, position);

const PLAYFIELD_WIDTH: f32 = 512.0;
const PLAYFIELD_HEIGHT: f32 = 384.0;

const NOTE_WIDTH: f32 = 0.1;
const NOTE_HEIGHT: f32 = 0.05;

fn main() {
	use osu_format::*;
	use glium::{DisplayBuild, Surface};

	let beatmap_dir = "/home/simon/.local/share/wineprefixes/osu/drive_c/users/simon/Local Settings/Application Data/osu!/Songs/171880 xi - Happy End of the World";
	let beatmap_path = &[beatmap_dir, "/xi - Happy End of the World (Blocko) [4K Easy].osu"].concat();

	let f = File::open(beatmap_path).unwrap();
	let r = BufReader::new(&f);
	let lines = r.lines();

	let mut p = Parser::new(lines);
	let beatmap = p.parse().unwrap();

	println!("{:?}", beatmap.general);
	println!("{:?}", beatmap.difficulty);

	let audio_path = &[beatmap_dir, "/", &beatmap.general.audio_filename].concat();

	let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

	let note_vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vertex{position: [0.0, 0.0]},
		Vertex{position: [0.0, NOTE_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, NOTE_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, 0.0]},
	]).unwrap();
	let note_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

	let keys_count: u32 = 4; // TODO
	let key_width = 1.0/(keys_count as f32);

	let per_instance = {
		#[derive(Debug, Copy, Clone)]
		struct Attr {
			at: u32,
			duration: u32,
			key: u32,
			milliseconds_per_beat: f32,
		}
		implement_vertex!(Attr, at, duration, key, milliseconds_per_beat);

		let data = beatmap.hit_objects.iter().map(|object| {
			let base = object.base();

			let duration: u32 = if let &HitObject::LongNote{end_time, ..} = object {
				end_time - base.time
			} else {
				0
			};

			Attr{
				at: base.time as u32,
				duration: duration,
				key: ((base.x as f32) / PLAYFIELD_WIDTH / key_width) as u32,
				milliseconds_per_beat: beatmap.timing_points[0].milliseconds_per_beat, // TODO
			}
		}).collect::<Vec<_>>();

		glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
	};

	let vertex_shader_src = include_str!("../shaders/note-vertex.glsl");
	let fragment_shader_src = include_str!("../shaders/note-fragment.glsl");
	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

	let endpoint = rodio::get_default_endpoint().unwrap();
	let sink = rodio::Sink::new(&endpoint);

	let file = std::fs::File::open(audio_path).unwrap();
	let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
	sink.append(source);

	let started_at = Instant::now();
	loop {
		let dur = Instant::now() - started_at;

		let uniforms = uniform!{
			time: (dur.as_secs() as u32)*1_000 + dur.subsec_nanos()/1_000_000,
			keys_count: keys_count,
		};

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(
			(&note_vertex_buffer, per_instance.per_instance().unwrap()),
			&note_indices, &program, &uniforms, &Default::default()
		).unwrap();
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) |
				Event::Closed => return,
				_ => (),
			}
		}
	}
}
