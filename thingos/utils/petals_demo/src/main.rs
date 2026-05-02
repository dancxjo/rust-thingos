#![no_std]
#![no_main]

extern crate alloc;

use petals::{
    AlignItems, Clock, Color, Declaration, Description, FlexDirection, JustifyContent,
    ResolvedStyle, Rule, Selector, UiTree,
};
use taffy::prelude::{AvailableSpace, Size};

#[stem::main]
fn main() -> ! {
    let status = run_demo();
    loop {
        stem::syscall::exit(status);
    }
}

fn run_demo() -> i32 {
    let mut tree = match UiTree::new() {
        Ok(tree) => tree,
        Err(err) => {
            stem::println!("petals_demo: create tree failed: {:?}", err);
            return 1;
        }
    };

    let button = match tree.pressable("Delete") {
        Ok(id) => id,
        Err(err) => {
            stem::println!("petals_demo: create button failed: {:?}", err);
            return 1;
        }
    };
    let text = match tree.text("Delete") {
        Ok(id) => id,
        Err(err) => {
            stem::println!("petals_demo: create text failed: {:?}", err);
            return 1;
        }
    };

    if let Err(err) = tree.add_child(tree.root(), button).and_then(|_| tree.add_child(button, text))
    {
        stem::println!("petals_demo: add child failed: {:?}", err);
        return 1;
    }

    let rules = alloc::vec![
        Rule::new(
            Selector::has(Description::Container),
            alloc::vec![Declaration::FlexDirection(FlexDirection::Column)],
        ),
        Rule::new(
            Selector::has(Description::Pressable),
            alloc::vec![
                Declaration::BackgroundColor(Color::rgb(0xb5, 0x89, 0x00)),
                Declaration::Padding(8.0),
                Declaration::BorderWidth(2.0),
                Declaration::FlexDirection(FlexDirection::Row),
                Declaration::JustifyContent(JustifyContent::Center),
                Declaration::Width(120.0),
                Declaration::Height(40.0),
            ],
        ),
        Rule::new(
            Selector::has(Description::Textual),
            alloc::vec![
                Declaration::Width(64.0),
                Declaration::Height(16.0),
                Declaration::FontSize(14.0)
            ],
        ),
    ];

    if let Err(err) = tree.restyle(&rules).and_then(|_| {
        tree.compute_layout(Size {
            width: AvailableSpace::Definite(320.0),
            height: AvailableSpace::Definite(200.0),
        })
    }) {
        stem::println!("petals_demo: layout failed: {:?}", err);
        return 1;
    }

    stem::println!("petals_demo: layout tree");
    for node in tree.nodes() {
        match tree.layout_box(node.id) {
            Ok(b) => stem::println!(
                "petals_demo: node={} desc={:?} box=({}, {}) {}x{}",
                node.id,
                node.descriptions,
                b.x as i32,
                b.y as i32,
                b.width as i32,
                b.height as i32
            ),
            Err(err) => stem::println!("petals_demo: node={} layout error: {:?}", node.id, err),
        }
    }

    if run_clock_petal_demo() != 0 {
        return 1;
    }

    stem::println!("petals_demo: PASS");
    0
}

fn run_clock_petal_demo() -> i32 {
    let clock = Clock::new();
    let state = clock.update_from_parts(2026, 5, 2, 21, 41);
    let time = clock.time_text(&state);
    let date = clock.date_text(&state);

    stem::println!(
        "petals_demo: clock petal time={} {} date={}",
        time,
        clock.am_pm_text(&state),
        date
    );

    let (mut tree, nodes) = match clock.build_tree(&state) {
        Ok(value) => value,
        Err(err) => {
            stem::println!("petals_demo: clock tree failed: {:?}", err);
            return 1;
        }
    };

    let root = tree.root();
    if let Err(err) = tree
        .apply_style(
            root,
            ResolvedStyle {
                width: Some(320.0),
                height: Some(200.0),
                flex_direction: Some(FlexDirection::Column),
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                ..ResolvedStyle::default()
            },
        )
        .and_then(|_| {
            tree.compute_layout(Size {
                width: AvailableSpace::Definite(320.0),
                height: AvailableSpace::Definite(200.0),
            })
        })
    {
        stem::println!("petals_demo: clock layout failed: {:?}", err);
        return 1;
    }

    match tree.global_layout_box(nodes.root) {
        Ok(b) => stem::println!(
            "petals_demo: clock root box=({}, {}) {}x{}",
            b.x as i32,
            b.y as i32,
            b.width as i32,
            b.height as i32
        ),
        Err(err) => {
            stem::println!("petals_demo: clock root layout error: {:?}", err);
            return 1;
        }
    }

    stem::println!("petals_demo: clock petal PASS");
    0
}
