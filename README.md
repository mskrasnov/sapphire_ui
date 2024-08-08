# Sapphire UI Kit

Sapphire UI Kit contains new themed components for [Iced](https://iced.rs). *Uses color schemes from [prettygooey](https://github.com/pieterdd/prettygooey)*. Sapphire UI Kit is primarily focused on the regular desktop (Linux, Windows, macOS).

![](assets/demo.png)

## Structure

The Crate is divided into two large parts: [`theme`](src/theme.rs) and [`widgets`](src/widgets.rs). The `theme` module contains only a styles, while `widgets` contains implementations of these widgets (as functions). Most functions are wrappers over Iced functions, but with modified styles applied. Sometimes it is allowed to change other non-styles parameters (such as widget sizes, fonts, etc.).

## Features

- `svg` - enables SVG support, adds `sapphire_ui::widgets::svg` widget;
- `qr_code` - enables QR-code generation support, adds `sapphire_ui::widgets::qr_code` widget;

## Installation

```toml
sapphire_ui = "0.1.0"
```

## Features

- [ ] Dark and Light styles;
- [ ] Support accent colors;
- [ ] New styles for:
    - [ ] Tabs (from `iced_aw`)
    - [ ] TabBar (from `iced_aw`)
    - [ ] scroll panel
    - [ ] checkboxes
    - [ ] radio buttons (!!!)
    - [ ] comboboxes
    - [ ] sliders
    - [ ] progress bars;
    - [ ] text editors;
    - [ ] togglers;
    - [ ] tooltips;
    - [ ] global menus (from `iced_aw`)
    - [ ] context menus (from `iced_aw`)
- [ ] New widgets:
    - [ ] About widget;
    - [ ] File Dialog widget:
        - [ ] Select single file/dir;
        - [ ] Select multiple files/dirs;
- [ ] Rewrite API

## Used in projects

- [resistor](https://github.com/mskrasnov/resistor) -  Программа для вычисления сопротивления резисторов по цветам их маркировки;
- [Cavaletto](https://github.com/mskrasnov/cavaletto) - AI program for image generation;

## License

Sapphire UI Kit distributed under MIT license.

## Support me

Users from Russia and Belarus can use [Boosty](https://boosty.to/linux-for-arm).
