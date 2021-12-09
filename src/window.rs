use crate::gen_maze::gen_maze;

#[allow(unused_imports)]
use crate::path_finders::{
    a_star::Astar, bfs::Bfs, dfs::Dfs, dijkstra::Dijkstra, path_finder::PathFinder,
};

use crate::util::Point3;

type Pf = Astar;

use std::time::Duration;

use druid::{
    widget::Flex, AppLauncher, Color, Data, Event, PlatformError, Point, Rect, RenderContext, Size,
    TimerToken, Widget, WindowDesc,
};

#[derive(Clone, Data)]
struct AppData {
    frames_per_second: f64,
    updates_per_second: f64,
    #[data(ignore)]
    path_finder: Pf,
}

impl AppData {
    fn new() -> Self {
        // let maze = vec![
        //     vec![true; 7],
        //     vec![false, false, false, false, true, false, true],
        //     vec![true, false, true, false, true, false, true],
        //     vec![true, false, true, true, true, false, true],
        //     vec![true, false, true, false, false, false, true],
        //     vec![true, false, false, false, true, false, false],
        //     vec![true; 7],
        // ];
        let (maze, end) = gen_maze(75, 75);

        AppData {
            frames_per_second: 20.0,
            updates_per_second: 40.0,
            path_finder: Pf::new(maze, Point3::new(0, 1), end),
        }
    }
    fn iter_interval(&self) -> u64 {
        (1000. / self.frames_per_second) as u64
    }

    pub fn tick_interval(&self) -> u64 {
        (1000. / (self.updates_per_second)) as u64
    }

    pub fn maze(&self) -> &Vec<Vec<bool>> {
        self.path_finder.get_maze()
    }
}

struct Canvas {
    paint_timer_id: TimerToken,
    tick_timer_id: TimerToken,
    //last_update: Instant,
}

impl Canvas {
    fn new() -> Self {
        Self {
            paint_timer_id: TimerToken::INVALID,
            //last_update: Instant::now(),
            tick_timer_id: TimerToken::INVALID,
        }
    }
}

fn make_rect(coords: Point3, width: f64, height: f64) -> Rect {
    Rect::from_points(
        Point::new(
            40. + coords.x as f64 * width,
            40. + coords.y as f64 * height,
        ),
        Point::new(
            40. + coords.x as f64 * width + width,
            40. + coords.y as f64 * height + width,
        ),
    )
}

impl Widget<AppData> for Canvas {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppData,
        _env: &druid::Env,
    ) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_millis(data.iter_interval());
                //self.last_update = Instant::now();
                self.paint_timer_id = ctx.request_timer(deadline);
                self.tick_timer_id = ctx.request_timer(Duration::from_millis(data.tick_interval()));
            }
            Event::Timer(id) => {
                if *id == self.paint_timer_id {
                    ctx.request_paint();
                    let deadline = Duration::from_millis(data.iter_interval());
                    self.paint_timer_id = ctx.request_timer(deadline);
                }
                if *id == self.tick_timer_id {
                    data.path_finder.step();

                    if !data.path_finder.done() {
                        self.tick_timer_id =
                            ctx.request_timer(Duration::from_millis(data.tick_interval()));
                    }
                }
                //self.last_update = Instant::now();
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &AppData,
        _env: &druid::Env,
    ) {
        // todo!()
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &AppData,
        _data: &AppData,
        _env: &druid::Env,
    ) {
        todo!()
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &AppData,
        _env: &druid::Env,
    ) -> druid::Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppData, _env: &druid::Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, ctx.size());
        ctx.fill(rect, &Color::rgb8(0x3B, 0x42, 0x52));

        let min_w = if ctx.size().height < ctx.size().width {
            ctx.size().height
        } else {
            ctx.size().width
        };
        let height = (min_w - 80.) / (data.maze().len() as f64);
        let width = height;
        for (i, row) in data.maze().iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                let color = if !*val {
                    Color::rgb8(0xE5, 0xE9, 0xF0)
                } else {
                    Color::rgb8(0x88, 0xC0, 0xD0)
                };
                ctx.fill(
                    make_rect(Point3::new(j as u16, i as u16), width, height),
                    &color,
                );
            }
        }
        for gekoloniseert in data.path_finder.get_progress() {
            let color = Color::rgb8(0xEB, 0xCB, 0x8B);
            ctx.fill(make_rect(*gekoloniseert, width, height), &color);
        }

        for estimated in data.path_finder.get_estimated_path() {
            let color = Color::rgb8(0x5E, 0x81, 0xAC);
            ctx.fill(make_rect(estimated, width, height), &color);
        }

        // // Dfs only
        // for each in data.path_finder.get_considered() {
        //     let color = Color::rgb8(0xA3, 0xBE, 0x8C);
        //     ctx.fill(make_rect(*each, width, height), &color);
        // }

        ctx.fill(
            make_rect(data.path_finder.end(), width, height),
            &Color::rgb8(0xBF, 0x61, 0x6A),
        );
        ctx.fill(
            Rect::from_points(
                Point::new(40., 40. + height),
                Point::new(40. + width, 40. + 2.0 * height),
            ),
            &Color::rgb8(0xD0, 0x87, 0x70),
        );
    }
}

fn make_widget() -> impl Widget<AppData> {
    Flex::column().with_flex_child(Canvas::new(), 1.0)
}

pub fn run() -> Result<(), PlatformError> {
    let appdata = AppData::new();
    let window = WindowDesc::new(make_widget)
        .window_size(Size {
            width: 800.0,
            height: 800.0,
        })
        .resizable(false)
        .title("confused snek");
    AppLauncher::with_window(window)
        // .log_to_console()
        .launch(appdata)
}
