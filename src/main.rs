#[macro_use] extern crate glium;
extern crate osu_format;
extern crate rodio;

mod difficulty;
mod hitline;
mod score;
mod timeline;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use difficulty::OverallDifficulty;
use hitline::HitLine;
use score::Score;
use timeline::Timeline;

#[derive(Debug, Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}
implement_vertex!(Vertex, position);

const PLAYFIELD_WIDTH: f32 = 512.0;
const PLAYFIELD_HEIGHT: f32 = 384.0;

const NOTE_WIDTH: f32 = 0.1;
const NOTE_HEIGHT: f32 = 0.05;

const KEY_HEIGHT: f32 = 0.2;

fn main() {
	use osu_format::{Parser, HitObject};
	use glium::{DisplayBuild, Surface, Blend};
	use glium::draw_parameters::{DrawParameters, BackfaceCullingMode};

	let beatmap_path = Path::new("/home/simon/.local/share/wineprefixes/osu/drive_c/users/simon/Local Settings/Application Data/osu!/Songs/171880 xi - Happy End of the World/xi - Happy End of the World (Blocko) [4K Easy].osu");
	let beatmap_dir_path = beatmap_path.parent().unwrap();

	let f = File::open(beatmap_path).unwrap();
	let r = BufReader::new(&f);
	let lines = r.lines();

	let mut p = Parser::new(lines);
	let beatmap = p.parse().unwrap();

	println!("{:?}", beatmap.general);
	println!("{:?}", beatmap.metadata);
	println!("{:?}", beatmap.difficulty);

	let audio_path = beatmap_dir_path.join(&beatmap.general.audio_filename);
	let keys_count = beatmap.difficulty.circle_size as u32;
	let overall_difficulty = OverallDifficulty::new(beatmap.difficulty.overall_difficulty);

	let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

	let note_vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vertex{position: [0.0, 0.0]},
		Vertex{position: [0.0, NOTE_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, NOTE_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, 0.0]},
	]).unwrap();
	let note_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

	let note_per_instance = {
		#[derive(Debug, Copy, Clone)]
		struct Attr {
			at: u32,
			duration: u32,
			key: u32,
			milliseconds_per_beat: f32,
		}
		implement_vertex!(Attr, at, duration, key, milliseconds_per_beat);

		let key_width = 1.0/(keys_count as f32);

		let mut timeline = Timeline::new(&beatmap.timing_points);
		let data = beatmap.hit_objects.iter().map(|object| {
			let base = object.base();

			let duration: u32 = if let &HitObject::LongNote{end_time, ..} = object {
				end_time - base.time
			} else {
				0
			};

			let point = timeline.at(base.time);

			Attr{
				at: base.time,
				duration: duration,
				key: ((base.x as f32) / PLAYFIELD_WIDTH / key_width) as u32,
				milliseconds_per_beat: point.milliseconds_per_beat,
			}
		}).collect::<Vec<_>>();

		glium::vertex::VertexBuffer::immutable(&display, &data).unwrap()
	};

	let note_program = {
		let vertex_shader_src = include_str!("../shaders/note-vertex.glsl");
		let fragment_shader_src = include_str!("../shaders/note-fragment.glsl");
		glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
	};

	let key_vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vertex{position: [0.0, 0.0]},
		Vertex{position: [0.0, KEY_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, KEY_HEIGHT]},
		Vertex{position: [NOTE_WIDTH, 0.0]},
	]).unwrap();
	let key_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

	let mut key_per_instance = {
		#[derive(Debug, Copy, Clone)]
		struct Attr {
			key: u32,
			pressed: u32,
		}
		implement_vertex!(Attr, key, pressed);

		let data = (0..keys_count).map(|key| {
			Attr{
				key: key,
				pressed: 0,
			}
		}).collect::<Vec<_>>();

		glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
	};

	let key_program = {
		let vertex_shader_src = include_str!("../shaders/key-vertex.glsl");
		let fragment_shader_src = include_str!("../shaders/note-fragment.glsl");
		glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
	};

	/*let endpoint = rodio::get_default_endpoint().unwrap();
	let sink = rodio::Sink::new(&endpoint);

	let file = std::fs::File::open(audio_path).unwrap();
	let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
	sink.append(source);*/

	let draw_parameters = DrawParameters{
		blend: Blend::alpha_blending(),
		backface_culling: BackfaceCullingMode::CullingDisabled,
		..DrawParameters::default()
	};

	let started_at = Instant::now();
	let mut timeline = Timeline::new(&beatmap.timing_points);
	let mut hit_line = HitLine::new(keys_count, overall_difficulty, &beatmap.hit_objects);
	let mut score = Score::new();
	loop {
		let dur = Instant::now() - started_at;
		let t = (dur.as_secs() as u32)*1_000 + dur.subsec_nanos()/1_000_000;
		let point = timeline.at(t);

		{
			let accuracy_list = hit_line.at(t as f32);
			if accuracy_list.len() > 0 {
				println!("{:?}", accuracy_list);
			}
			for acc in accuracy_list {
				score.push(acc);
			}
		}

		let note_uniforms = uniform!{
			keys_count: keys_count,
			current_time: t,
			current_milliseconds_per_beat: point.milliseconds_per_beat,
		};

		let key_uniforms = uniform!{
			keys_count: keys_count,
		};

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(
			(&note_vertex_buffer, note_per_instance.per_instance().unwrap()),
			&note_indices, &note_program, &note_uniforms, &draw_parameters
		).unwrap();
		target.draw(
			(&key_vertex_buffer, key_per_instance.per_instance().unwrap()),
			&key_indices, &key_program, &key_uniforms, &draw_parameters
		).unwrap();
		target.finish().unwrap();

		for ev in display.poll_events() {
			match ev {
				Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) |
				Event::Closed => {
					println!("Accuracy: {}", score.accuracy());
					println!("Score: {}", score.score());
					return;
				},
				Event::KeyboardInput(state, _, Some(keycode)) => {
					let key = match keycode {
						VirtualKeyCode::D => Some(0),
						VirtualKeyCode::F => Some(1),
						VirtualKeyCode::J => Some(2),
						VirtualKeyCode::K => Some(3),
						_ => None,
					};

					if let Some(key) = key {
						let pressed = match state {
							ElementState::Pressed => 1,
							ElementState::Released => 0,
						};

						let mut mapping = key_per_instance.map();
						mapping.iter_mut().nth(key as usize).unwrap().pressed = pressed;

						let acc = if state == ElementState::Pressed {
							hit_line.press(key)
						} else {
							hit_line.release(key)
						};

						if let Some(acc) = acc {
							println!("{:?}", acc);
							score.push(acc);
						}
					}
				},
				_ => (),
			}
		}
	}
}
