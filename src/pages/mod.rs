//! Re-exports for all page components.
pub mod about;
pub mod contact;
pub mod home;
pub mod not_found;
pub mod one_pager;
pub mod project;
pub mod resume;
pub mod telemetry;
pub mod writing;

pub use about::AboutPage;
pub use contact::ContactPage;
pub use home::HomePage;
pub use not_found::NotFoundPage;
pub use one_pager::OnePageSummary;
pub use project::{ProjectDemoPage, ProjectDetailPage, ProjectDocsPage};
pub use resume::ResumePage;
pub use telemetry::TelemetryPage;
pub use writing::{WriteupDetailPage, WritingPage};
