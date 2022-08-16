use core::f32::consts::PI;

/// Width (in pixel) of the calculator screen
pub const SCREEN_WIDTH: u16 = 320;

/// Height (in pixel) of the calculator screen
pub const SCREEN_HEIGHT: u16 = 240;

/// Wrapper arround a color
///
/// Colors are encoded using rgb565.
/// Where 5 bits are used for red, 6 for green and 5 for blue.
/// So colors can be encoded in a u16 instead of 3 u8 for usual rgb888
///
/// # Example
///
/// ```
/// use eadk::Color;
///
/// let color = Color { rgb565: 0b1111_1000_0000_0000 /* red */ };
/// ```
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    pub const BLACK: Color = Self::from_rgb888(0, 0, 0);
    pub const WHITE: Color = Self::from_rgb888(255, 255, 255);
    pub const RED: Color = Self::from_rgb888(255, 0, 0);
    pub const GREEN: Color = Self::from_rgb888(0, 255, 0);
    pub const BLUE: Color = Self::from_rgb888(0, 0, 255);

    /// Create color from rgb565
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::Color;
    ///
    /// let color = Color::new(0b1111_1000_0000_0000); // red
    /// ```
    #[must_use]
    pub const fn new(rgb565: u16) -> Self {
        Self { rgb565 }
    }

    /// Create color from rgb888
    ///
    /// Perform conversion from rgb888 to rgb565
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::Color;
    ///
    /// let color = Color::from_rgb888(255,0,0); // red
    /// ```
    #[must_use]
    pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb565: ((r as u16 & 0b1111_1000) << 8)
                | ((g as u16 & 0b1111_1100) << 3)
                | (b as u16 >> 3),
        }
    }

    /// Create color from hsv
    ///
    /// Perform conversion from hsv to rgb565.
    /// Hue is in radian, saturation and value are in the [0; 1] range.
    ///
    /// # Example
    ///
    /// ```
    /// use core::f32::consts::PI;
    /// use eadk::Color;
    ///
    /// let color = Color::from_hsv(PI, 1., 1.); // green
    /// ```
    #[must_use]
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let f = |n: f32| {
            let k: f32 = (n + hue / PI * 3.) % 6.;
            value * (1. - saturation * k.min(4. - k).min(1.).max(0.))
        };
        Color::from_rgb888(
            (f(5.) * 255.) as u8,
            (f(3.) * 255.) as u8,
            (f(1.) * 255.) as u8,
        )
    }
}

/// A rectangle on the screen
///
/// # Example
///
/// ```
/// use eadk::{display, Rect, Color};
///
/// display::push_rect_uniform(Rect {
///     x: 0,
///     y: 0,
///     width: 10,
///     height: 10
/// }, Color::RED);
/// ```
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    /// Full screen rectangle
    pub const SCREEN: Self = Self::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    /// Create a rectangle
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{display, Rect, Color};
    ///
    /// display::push_rect_uniform(Rect::new(0, 0, 10, 10), Color::RED);
    /// ```
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// A point on the screen
///
/// # Example
///
/// ```
/// use eadk::{Point, SCREEN_WIDTH, SCREEN_HEIGHT};
///
/// let center = Point {
///     x: SCREEN_WIDTH / 2,
///     y: SCREEN_HEIGHT / 2,
/// };
/// ```
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    /// The top left corner
    pub const ZERO: Self = Self::new(0, 0);

    /// Create a new point
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::Point;
    ///
    /// let p = Point::new(10, 10);
    /// ```
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Wrapper arround a keyboard state
///
/// This type is the result of a [keyboard scan](keyboard::scan).
/// Internaly, the keyboard state is a u64.
/// So to query key state, use [`State::key_down`] with a [key](key).
///
/// # Example
///
/// ```
/// use eadk::{keyboard, State};
///
/// let keyboard_state: State = keyboard::scan();
/// ```
#[repr(C)]
pub struct State(u64);

impl State {
    #[must_use]
    fn new(state: u64) -> Self {
        Self(state)
    }

    /// Query the internal keyboard state
    ///
    /// Use a [key](key) constant as parameter.
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{keyboard, key};
    ///
    /// let keyboard_state = keyboard::scan();
    /// if keyboard_state.key_down(key::EXE) {
    ///     todo!();
    /// }
    /// ```
    #[must_use]
    pub fn key_down(&self, k: u32) -> bool {
        self.0.wrapping_shr(k) & 1 != 0
    }
}

/// Key constants
///
/// Contain all key constants used to query [keyboard state](State).
pub mod key {
    pub const LEFT: u32 = 0;
    pub const UP: u32 = 1;
    pub const DOWN: u32 = 2;
    pub const RIGHT: u32 = 3;
    pub const OK: u32 = 4;
    pub const BACK: u32 = 5;
    pub const HOME: u32 = 6;
    pub const SHIFT: u32 = 12;
    pub const ALPHA: u32 = 13;
    pub const XNT: u32 = 14;
    pub const VAR: u32 = 15;
    pub const TOOLBOX: u32 = 16;
    pub const BACKSPACE: u32 = 17;
    pub const EXP: u32 = 18;
    pub const LN: u32 = 19;
    pub const LOG: u32 = 20;
    pub const IMAGINARY: u32 = 21;
    pub const COMMA: u32 = 22;
    pub const POWER: u32 = 23;
    pub const SINE: u32 = 24;
    pub const COSINE: u32 = 25;
    pub const TANGENT: u32 = 26;
    pub const PI: u32 = 27;
    pub const SQRT: u32 = 28;
    pub const SQUARE: u32 = 29;
    pub const SEVEN: u32 = 30;
    pub const EIGHT: u32 = 31;
    pub const NINE: u32 = 32;
    pub const LEFTPARENTHESIS: u32 = 33;
    pub const RIGHTPARENTHESIS: u32 = 34;
    pub const FOUR: u32 = 36;
    pub const FIVE: u32 = 37;
    pub const SIX: u32 = 38;
    pub const MULTIPLICATION: u32 = 39;
    pub const DIVISION: u32 = 40;
    pub const ONE: u32 = 42;
    pub const TWO: u32 = 43;
    pub const THREE: u32 = 44;
    pub const PLUS: u32 = 45;
    pub const MINUS: u32 = 46;
    pub const ZERO: u32 = 48;
    pub const DOT: u32 = 49;
    pub const EE: u32 = 50;
    pub const ANS: u32 = 51;
    pub const EXE: u32 = 52;
}

/// Get and set calculator brightness
///
/// Changes to the brighness aren't persistents and will be reverted if you quit the app.
///
/// # Example
///
/// ```
/// use eadk::backlight;
///
/// backlight::set_brightness(0); // minimal brightness
///```
pub mod backlight {
    /// Set calculator brightness
    ///
    /// The brightness goes from 0 (minimum brighness) to 255 (maximum).
    /// After quiting the app, brighness changes aren't persisted.
    ///
    /// # Example
    /// ```
    /// use eadk::backlight;
    ///
    /// backlight::set_brightness(255); // Maximum brightness
    /// ```
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }

    /// Get calculator brightness
    ///
    /// The return value goes from 0 (minimal brightness) to 255 (maximum brightness).
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::backlight;
    ///
    /// backlight::brightness(); // [0; 255]
    /// ```
    pub fn brightness() -> u8 {
        unsafe {
            return eadk_backlight_brightness();
        }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }
}

/// Draw on the screen
///
/// Draw string and fill rects.
///
/// # Example
///
/// ```
/// use eadk::{display, Color, Rect};
///
/// display::push_rect_uniform(Rect::SCREEN, Color::BLUE); // Fill the screen in blue
/// ```
pub mod display {
    use super::Color;
    use super::Point;
    use super::Rect;

    /// Push a frame rect to the frame buffer
    ///
    /// For a [rect](Rect), push an array of pixel colors.
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{display, Rect, Color};
    ///
    /// // Set colors of the pixel in the 2x2 rect at x = 10, y = 10
    /// display::push_rect(
    ///     Rect::new(10, 10, 2, 2),
    ///     &[Color::RED, Color::GREEN, Color::BLUE, Color::WHITE]
    /// );
    /// ```
    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    /// Set the color of all the pixels in a rect
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{display, Rect, Color};
    ///
    /// display::push_rect(Rect::new(0, 0, 100, 100), Color::BLACK);
    /// ```
    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    /// Draw a string on the screen
    ///
    /// *The string must end with the '\0' character*
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{display, Point, Color};
    ///
    /// display::draw_string("Hello numworks!\0", Point::ZERO, true, Color::BLACK, Color::WHITE);
    /// ```
    pub fn draw_string(
        string: &str,
        pos: Point,
        large: bool,
        text_color: Color,
        background_color: Color,
    ) {
        unsafe {
            eadk_display_draw_string(string.as_ptr(), pos, large, text_color, background_color);
        }
    }

    /// Wait for vertical blanking
    ///
    /// Usefull when you draw on screen in loops to avoid screen blinking.
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::{display, Rect, Color};
    ///
    /// let mut x = 0;
    /// loop {
    ///     x += 1;
    ///     display::push_rect_uniform(Rect::new(x,10,10,10), Color::RED);
    ///     display::wait_for_vblank();
    /// }
    /// ```
    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_draw_string(
            text: *const u8,
            pos: Point,
            large: bool,
            text_color: Color,
            background_color: Color,
        );
        fn eadk_display_wait_for_vblank();
    }
}

/// Scan the keyboard state
///
/// # Example
///
/// ```
/// use eadk::keyboard;
///
/// let keyboard_state = keyboard::scan();
/// ```
pub mod keyboard {
    use super::State;

    /// Scan the keyboard state
    ///
    /// See [keyboard state](State)
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::keyboard;
    ///
    /// let keyboard_state = keyboard::scan();
    /// ```
    #[must_use]
    pub fn scan() -> State {
        unsafe { State::new(eadk_keyboard_scan()) }
    }

    extern "C" {
        fn eadk_keyboard_scan() -> u64;
    }
}

/// Timing related functions
///
/// # Example
///
/// ```
/// use eadk::timing;
///
/// timing::msleep(1000); // sleep 1s
/// ```
pub mod timing {
    /// Sleep in microseconds
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::timing;
    ///
    /// timing::usleep(1_000_000); // Sleep 1s
    /// ```
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    /// Sleep in milliseconds
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::timing;
    ///
    /// timing::usleep(1_000); // Sleep 1s
    /// ```
    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    /// Get time elapsed in milliseconds
    ///
    /// # Example
    ///
    /// ```
    /// use eadk::timing;
    ///
    /// let before = timing::millis();
    /// timing::msleep(500);
    /// let after = timing::millis();
    /// after - before; // Time elapsed ~500
    pub fn millis() -> u64 {
        unsafe {
            return eadk_timing_millis();
        }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

/// Get a random u32
///
/// Get a random number in [0; 4_294_967_295].
/// # Examples
///
/// Basic usage:
/// ```
/// use eadk::random;
///
/// let r: u32 = random();
/// ```
///
/// Number from 0 to n:
/// ```
/// use eadk::random;
///
/// let r: u32 = random() % n;
/// ```
pub fn random() -> u32 {
    unsafe { return eadk_random() }
}

extern "C" {
    fn eadk_random() -> u32;
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    display::push_rect_uniform(Rect::SCREEN, Color::RED);
    display::draw_string(
        "Error !\0",
        Point::new(10, 10),
        true,
        Color::BLACK,
        Color::WHITE,
    );

    loop {}
}
