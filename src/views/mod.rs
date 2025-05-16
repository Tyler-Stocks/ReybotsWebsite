//! The views module contains the data for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these data.
//!
//!
//! The [`Home`] and [`Blog`] data will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::Home;

mod nav;
pub use nav::Nav;

mod competitions;
pub use competitions::Competitions;

mod sponsors;
pub use sponsors::Sponsors;

mod sponsor_us;
pub use sponsor_us::SponsorUs;

mod contact;
pub use contact::Contact;

mod about;
pub use about::About;

mod footer;
pub use footer::Footer;

