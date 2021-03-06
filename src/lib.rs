//! Thirtyfour_query provides an advanced query interface for `thirtyfour`, featuring
//! powerful filtering and polling options.
//!
//! See examples for more details.
//!
//! ## Usage
//!
//! First, set the default polling behaviour:
//! ```no_run
//! // Disable implicit timeout in order to use new query interface.
//! driver.set_implicit_wait_timeout(Duration::new(0, 0)).await?;
//!
//! let poller = ElementPoller::TimeoutWithInterval(Duration::new(20, 0), Duration::from_millis(500));
//! driver.config_mut().set("ElementPoller", poller)?;
//! ```
//!
//! Other ElementPoller options are also available, such as NoWait and NumTriesWithInterval.
//! These can be overridden on a per-query basis as needed.
//!
//! Now, using the query interface you can do things like:
//!
//! ```no_run
//! let elem_text =
//!     driver.query(By::Css("thiswont.match")).or(By::Id("searchInput")).first().await?;
//! ```
//!
//! This will execute both queries once per poll iteration and return the first one that matches.
//! You can also filter on one or both match arms like this:
//!
//! ```no_run
//! driver.query(By::Css("thiswont.match")).with_text("testing")
//!     .or(By::Id("searchInput")).with_class("search").and_not_enabled()
//!     .first().await?;
//! ```
//!
//! To fetch all matching elements instead of just the first one, simply change first() to all()
//! and you'll get a Vec instead. This will never return an empty Vec. If either first() or all()
//! don't match anything, you'll get `WebDriverError::NoSuchElement` instead.
//! The error message will show the selectors used.
//!
pub mod query;
