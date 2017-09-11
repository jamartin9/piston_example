#[macro_use]
extern crate clap;
extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate fps_counter;
extern crate sdl2_window;
extern crate music;
extern crate sdl2;

//TODO: Menu/UI ( Conrod ? )
//TODO: Physics ( nphysics ? )
//TODO: enemies spawns
fn main() {
    use sdl2_window::Sdl2Window;
    use fps_counter::FPSCounter;
    use std::rc::Rc;
    use piston_window::*;
    use sprite::*;
    use ai_behavior::{Action,
                      Sequence,
                      //    Wait,
                      WaitForever,
                      While};

    // get cli args
    use clap::App;

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let fullscreen = matches.is_present("fullscreen");
    let resizeable = matches.is_present("resizeable");
    let mut width = matches
        .value_of("width")
        .unwrap_or("640")
        .parse()
        .expect("expected a number");
    let mut height = matches
        .value_of("height")
        .unwrap_or("480")
        .parse()
        .expect("expected a number");

    // make zero'd out window
    let mut piston_window: PistonWindow<Sdl2Window> = WindowSettings::new("piston game", (width, height))
        .exit_on_esc(true)
        .resizable(true)
        .opengl(OpenGL::V3_2)
        .vsync(true)
        .samples(4)
        .fullscreen(fullscreen)
        .build()
        .unwrap();

    piston_window.set_position((0, 0));
    if fullscreen {
        width = piston_window.draw_size().width;
        height = piston_window.draw_size().height;
    }

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    // sdl context
    let sdl = piston_window.window.sdl_context.to_owned();
    let ev = sdl.event().unwrap();

    // get font assets
    let font = &assets.join("FiraCode-Regular-modified.ttf");
    let factory = piston_window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    // create sprites
    let background_tex = Rc::new(Texture::from_path(&mut piston_window.factory,
                                                    assets.join("figure_1.png"),
                                                    Flip::None,
                                                    &TextureSettings::new())
                                         .unwrap());
    let mut background = Sprite::from_texture(background_tex.clone());
    background.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let player_tex = Rc::new(Texture::from_path(&mut piston_window.factory,
                                                assets.join("rust.png"),
                                                Flip::None,
                                                &TextureSettings::new())
                                     .unwrap());
    let mut player_sprite = Sprite::from_texture(player_tex.clone());
    player_sprite.set_position(0.0, 0.0);
    let player_id = background.add_child(player_sprite);

    // create scene
    let mut scene = Scene::new();
    //let background_id =
    scene.add_child(background);

    // create animations
    let blink = While(Box::new(WaitForever),
                      vec![Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
                           Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0))))]);
    let left_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                              Box::new(MoveBy(1.0, -32.0, 0.0))))]);
    let right_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                               Box::new(MoveBy(1.0, 32.0, 0.0))))]);
    let up_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                            Box::new(MoveBy(1.0, 0.0, -32.0))))]);
    let down_anim = Sequence(vec![Action(Ease(EaseFunction::BounceOut,
                                              Box::new(MoveBy(1.0, 0.0, 32.0))))]);

    // init scene setup
    scene.run(player_id, &blink);
    scene.toggle(player_id, &blink);

    // fps setup
    let mut fps_counter = FPSCounter::new();
    let mut show_fps = false;

    // sdl audio
    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    enum Music {
        Piano,
    }
    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    enum Sound {
        Ding,
    }

    music::start_context::<Music, Sound, _>(&sdl, 16, || {
        music::bind_music_file(Music::Piano, "./assets/piano.wav");
        music::bind_sound_file(Sound::Ding, "./assets/ding.wav");
        //music::play_music(&Music::Piano, music::Repeat::Forever);
        //music::play_sound(&Sound::Ding, music::Repeat::Times(1));

        // window events
        while let Some(e) = piston_window.next() {
            let fps = fps_counter.tick();
            scene.event(&e);
            // resize
            if let Input::Resize(w, h) = e {
                width = w;
                height = h;
                println!("Resize: {:?} {:?}", width, height);
                println!("Resize Window: {:?}",(piston_window.draw_size().width, piston_window.draw_size().height));
                println!("Resize Drawable: {:?}",piston_window.window.window.drawable_size());
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::F => {
                        println!("F1 Window: {:?}",(piston_window.draw_size().width, piston_window.draw_size().height));
                        println!("F1 Drawable: {:?}",piston_window.window.window.drawable_size());
                        // sets SDL size
                        piston_window.window.window.set_size(480,480);
                        // updates drawable size in piston/SDL
                        let event = sdl2::event::Event::Window {
                            timestamp: (0),
                            window_id: (0),
                            win_event: ( sdl2::event::WindowEvent::Resized(480,480)),
                        };
                        ev.push_event(event);
                        //window.window.window.maximize();
                        //scene.child_mut(background).unwrap().set_scale(0.5, 0.5);

                        // toggle fps
                        if show_fps {
                            show_fps = false;
                        } else {
                            show_fps = true;
                        }
                    }
                    Key::A => scene.run(player_id, &left_anim),
                    Key::D => scene.run(player_id, &right_anim),
                    Key::S => scene.run(player_id, &down_anim),
                    Key::W => scene.run(player_id, &up_anim),
                    Key::Space => {
                        println!("{:?}", scene.child(player_id).unwrap().get_position());
                        scene.toggle(player_id, &blink)
                    }
                    _ => println!("Unregistered keyboard key '{:?}'", key),
                }
            }
            piston_window.draw_2d(&e, |c, g| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                scene.draw(c.transform, g);
                // show fps
                if show_fps {
                    let fps_display_string = &*fps.to_string();
                    let transform = c.transform.trans(10.0, 30.0);
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                        .draw(fps_display_string, &mut glyphs, &c.draw_state, transform, g);
                }
            });
        }
    });
}
