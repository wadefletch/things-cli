use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "things", about = "CLI for Things 3", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show today's tasks
    Today,

    /// Show inbox tasks
    Inbox,

    /// Show upcoming scheduled tasks
    Upcoming,

    /// Show someday tasks
    Someday,

    /// Show completed tasks
    Logbook {
        /// Show tasks completed since this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        /// Maximum number of tasks to show
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// List tasks with filters
    List {
        /// Filter by project name
        #[arg(long)]
        project: Option<String>,

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Filter by area
        #[arg(long)]
        area: Option<String>,

        /// Only show tasks with deadlines
        #[arg(long)]
        deadline: bool,
    },

    /// Show task details
    Show {
        /// Task ID (UUID prefix) or title substring
        id: String,
    },

    /// Search tasks
    Search {
        /// Search query
        query: String,

        /// Include completed tasks
        #[arg(long)]
        include_completed: bool,
    },

    /// Add a new task via Things URL scheme
    Add {
        /// Task title
        title: String,

        /// Notes
        #[arg(long)]
        notes: Option<String>,

        /// When to start (today, tomorrow, evening, someday, or YYYY-MM-DD)
        #[arg(long = "when")]
        when_date: Option<String>,

        /// Deadline (YYYY-MM-DD)
        #[arg(long)]
        deadline: Option<String>,

        /// Comma-separated tags
        #[arg(long)]
        tags: Option<String>,

        /// Target project or area
        #[arg(long)]
        list: Option<String>,

        /// Heading within project
        #[arg(long)]
        heading: Option<String>,

        /// Comma-separated checklist items
        #[arg(long)]
        checklist: Option<String>,

        /// Reveal task in Things after creating
        #[arg(long)]
        reveal: bool,
    },

    /// Complete or cancel a task
    Complete {
        /// Task ID (UUID prefix) or title substring
        id: String,

        /// Cancel instead of completing
        #[arg(long)]
        cancel: bool,
    },

    /// List projects
    Projects {
        /// Filter by area
        #[arg(long)]
        area: Option<String>,
    },

    /// Show project detail and its tasks
    Project {
        /// Project name
        name: String,
    },

    /// List areas
    Areas,

    /// List tags
    Tags,

    /// Manage auth token
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
}

#[derive(Subcommand)]
pub enum AuthAction {
    /// Store auth token
    Set {
        /// The auth token
        token: String,
    },
    /// Show masked auth token
    Show,
    /// Remove auth token
    Clear,
}
