#[macro_export]
macro_rules! print_success {
    ($($data:expr),*) => {{
        let binding: String = format!("[{} => success]",current_time());
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color(ColorType::Rgb(0,255,0))
            .color(ColorType::Color256(0xffffff))
            .build();
        let mut output_list_builder = OutputListBuilder::new();
        output_list_builder.add(time_output);
        $(
            let text = $data.to_string();
            let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
            let text_output: Output<'_> = text_output_builder
                .text(&text)
                .blod(true)
                .color(ColorType::Rgb(0,255,0))
                .endl(false)
                .build();
            output_list_builder.add(text_output);
        )*
        output_list_builder.add(text_endl_output);
        output_list_builder.run();
    }};
}

#[macro_export]
macro_rules! print_warning {
    ($($data:expr),*) => {{
        let binding: String = format!("[{} => warning]",current_time());
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color(ColorType::Rgb(255, 255, 0))
            .color(ColorType::Color256(0xffffff))
            .build();
        let mut output_list_builder = OutputListBuilder::new();
        output_list_builder.add(time_output);
        $(
            let text = $data.to_string();
            let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
            let text_output: Output<'_> = text_output_builder
                .text(&text)
                .blod(true)
                .color(ColorType::Rgb(255, 255, 0))
                .endl(false)
                .build();
            output_list_builder.add(text_output);
        )*
        output_list_builder.add(text_endl_output);
        output_list_builder.run();
    }};
}

#[macro_export]
macro_rules! print_danger {
    ($($data:expr),*) => {{
        let binding: String = format!("[{} => danger]",current_time());
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color(ColorType::Rgb(255,0,0))
            .color(ColorType::Color256(0xffffff))
            .build();
        let mut output_list_builder = OutputListBuilder::new();
        output_list_builder.add(time_output);
        $(
            let text = $data.to_string();
            let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
            let text_output: Output<'_> = text_output_builder
                .text(&text)
                .blod(true)
                .color(ColorType::Rgb(255,0,0))
                .endl(false)
                .build();
            output_list_builder.add(text_output);
        )*
        output_list_builder.add(text_endl_output);
        output_list_builder.run();
    }};
}
