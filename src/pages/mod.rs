//! The pages module contains the data_types for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these data_types.
//!
//!
//! The [`HomePage`] and [`Blog`] data_types will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::HomePage;

mod competitions;
pub use competitions::CompetitionsPage;
mod sponsors;
pub use sponsors::SponsorsPage;
mod sponsor_us;
pub use sponsor_us::SponsorUsPage;
mod contact;
pub use contact::ContactPage;
mod about;
pub use about::AboutPage;

