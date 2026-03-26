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

    /// Show completed and canceled tasks
    Logbook {
        /// Only show tasks completed since this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        /// Maximum number of tasks to show
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// List tasks with filters
    List {
        /// Filter by project
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

    /// Show details for a task or project
    Show {
        /// Ref (e.g. t1, p2) or UUID prefix
        id: String,
    },

    /// Search tasks by title
    Search {
        /// Search query
        query: String,

        /// Include completed tasks in results
        #[arg(long)]
        include_completed: bool,
    },

    /// Add a new task
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

        /// Project or area to add to
        #[arg(long)]
        list: Option<String>,

        /// Heading within project
        #[arg(long)]
        heading: Option<String>,

        /// Comma-separated checklist items
        #[arg(long)]
        checklist: Option<String>,

        /// Reveal in Things after creating
        #[arg(long)]
        reveal: bool,
    },

    /// Add a new project
    AddProject {
        /// Project title
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

        /// Area to add to
        #[arg(long)]
        area: Option<String>,

        /// Comma-separated to-do items to create inside the project
        #[arg(long)]
        todos: Option<String>,

        /// Reveal in Things after creating
        #[arg(long)]
        reveal: bool,
    },

    /// Edit a task or project
    Edit {
        /// Ref (e.g. t1, p2) or UUID prefix
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New notes (replaces existing)
        #[arg(long)]
        notes: Option<String>,

        /// When to start (today, tomorrow, evening, someday, or YYYY-MM-DD)
        #[arg(long = "when")]
        when_date: Option<String>,

        /// Deadline (YYYY-MM-DD)
        #[arg(long)]
        deadline: Option<String>,

        /// Comma-separated tags (replaces existing)
        #[arg(long)]
        tags: Option<String>,

        /// Project or area to move to
        #[arg(long)]
        list: Option<String>,

        /// Heading within project
        #[arg(long)]
        heading: Option<String>,

        /// Comma-separated checklist items to append
        #[arg(long)]
        checklist_append: Option<String>,

        /// Comma-separated checklist items to prepend
        #[arg(long)]
        checklist_prepend: Option<String>,

        /// Reveal in Things after editing
        #[arg(long)]
        reveal: bool,
    },

    /// Complete or cancel a task or project
    Complete {
        /// Ref (e.g. t1, p2) or UUID prefix
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

    /// Show a project and its tasks
    Project {
        /// Project ref (e.g. p1), UUID prefix, or name
        name: String,
    },

    /// List areas
    Areas,

    /// List tags
    Tags,

    /// Show or clear refs
    Refs {
        /// Clear all refs
        #[arg(long)]
        clear: bool,
    },

    /// Manage auth token for write operations
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
    /// Show current auth token (masked)
    Show,
    /// Remove auth token
    Clear,
}
