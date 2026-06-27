use crate::utils::run_hooks;
use chrono::Local;
use inquire::error::InquireResult;
use inquire::{Confirm, Editor, InquireError, Select, Text};
#[cfg(unix)]
use nix::sys::utsname::uname;
#[cfg(unix)]
use nix::unistd::User;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};
use std::io::Error;

pub const WHY_PROMPT: &str = "Explain the reason for this change";
pub const HOW_PROMPT: &str = "Details the changes";
pub const SUBJECT_PROMPT: &str = "Summary of changes";
pub const OUTCOME_PROMPT: &str = "Outcome of changes";

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct CommitType {
    pub name: &'static str,
    pub mnemonic: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommitCategory {
    CoreChanges,
    MaintenanceInfrastructure,
    ProjectEvents,
    CommunicationCollaboration,
    CelestialEvents,
    CelestialObjects,
    AstronomicalConcepts,
    SpaceExploration,
}

impl Default for CommitCategory {
    fn default() -> Self {
        Self::CoreChanges
    }
}

impl Default for CommitType {
    fn default() -> Self {
        Self {
            name: "Star",
            mnemonic: "Shiny Technology Added or Refined",
            description: "New feature or enhancement",
            example: "Star(Auth): Implement two-factor authentication",
        }
    }
}

impl Display for CommitCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.as_str())?;
        Ok(())
    }
}

impl Display for CommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.description, self.name)?;
        Ok(())
    }
}

impl CommitCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CoreChanges => "Core Changes",
            Self::MaintenanceInfrastructure => "Maintenance & Infrastructure",
            Self::ProjectEvents => "Project Events",
            Self::CommunicationCollaboration => "Communication & Collaboration",
            Self::CelestialEvents => "Celestial Events",
            Self::CelestialObjects => "Celestial Objects",
            Self::AstronomicalConcepts => "Astronomical Concepts",
            Self::SpaceExploration => "Space Exploration",
        }
    }

    pub fn all() -> Vec<Self> {
        Vec::from([
            Self::CoreChanges,
            Self::MaintenanceInfrastructure,
            Self::ProjectEvents,
            Self::CommunicationCollaboration,
            Self::CelestialEvents,
            Self::CelestialObjects,
            Self::AstronomicalConcepts,
        ])
    }
}

// Macro utilitaire pour instancier rapidement les CommitType
macro_rules! commit_type {
    ($name:expr, $mnemonic:expr, $desc:expr, $ex:expr) => {
        CommitType {
            name: $name,
            mnemonic: $mnemonic,
            description: $desc,
            example: $ex,
        }
    };
}

// Utilisation d'un lazy_static ou simplement d'une fonction de construction
// pour générer la map des thèmes spatiaux
pub fn get_space_themes() -> HashMap<CommitCategory, Vec<CommitType>> {
    let mut themes = HashMap::new();

    themes.insert(
        CommitCategory::CoreChanges,
        vec![
            commit_type!(
                "Star",
                "Shiny Technology Added or Refined",
                "New feature or enhancement",
                "Star(Auth): Implement two-factor authentication"
            ),
            commit_type!(
                "Comet",
                "Code or Module Error Terminated",
                "Bug fix or error resolution",
                "Comet(UI): Fix responsive layout issue on mobile devices"
            ),
            commit_type!(
                "Nebula",
                "New Efficient Better Understandable Logic Achieved",
                "Code refactoring",
                "Nebula(Backend): Refactor user management module for improved maintainability"
            ),
            commit_type!(
                "Pulsar",
                "Powerful Upgrade, Less Sluggish, Agile Response",
                "Performance improvement",
                "Pulsar(Database): Optimize queries for faster response times"
            ),
            commit_type!(
                "Quasar",
                "Quick Adjustments for Superior Accuracy and Readability",
                "Documentation or clarity improvement",
                "Quasar(API): Update documentation with new endpoint parameters"
            ),
        ],
    );

    themes.insert(
        CommitCategory::MaintenanceInfrastructure,
        vec![
            commit_type!("Asteroid Belt", "Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat", "Code cleanup and maintenance", "Asteroid Belt: Remove unused CSS and optimize images"),
            commit_type!("Solar Flare", "Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured", "Adding or updating tests (unit, integration, end-to-end).", "Solar Flare(Payments): Add unit tests for payment processing module"),
            commit_type!("Dwarf Planet", "Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced Tiny", "Minor but essential updates or fixes.", "Dwarf Planet: Update project dependencies to latest versions"),
            commit_type!("Terraform", "Technology Engineering Resources Readily Automated, Foundation of Reliable Management", "Infrastructure changes", "Terraform(AWS): Provision new EC2 instance for staging environment"),
        ],
    );

    themes.insert(
        CommitCategory::ProjectEvents,
        vec![
            commit_type!(
                "Black Hole",
                "Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely",
                "Removing large chunks of code or features",
                "Black Hole: Remove deprecated user profile module"
            ),
            commit_type!(
                "Wormhole",
                "Weaving or Reconnecting Modules, Hitching onto Linked Elements",
                "Merging branches or connecting code parts",
                "Wormhole: Merge feature/new-dashboard into develop branch"
            ),
            commit_type!(
                "Big Bang",
                "Birth of Initial Greatness, Beginning All New Growth",
                "Initial commit of a project or major feature",
                "Big Bang: Initial project setup and scaffolding"
            ),
            commit_type!(
                "Launch",
                "Lifting Application Upward, New Code Entering Production",
                "Deploying to production or releasing a version",
                "Launch(v1.2): Release new version with user profile customization"
            ),
        ],
    );

    themes.insert(
        CommitCategory::CommunicationCollaboration,
        vec![
            commit_type!("Lightspeed", "Lightening Speed Enhancements", "Significant performance improvements", "Lightspeed(Frontend): Implement lazy loading for images"),
            commit_type!("Mission Control", "Managing Changes, Issues, Scope, Teamwork, and Release On Time", "Project management changes", "Mission Control: Update project roadmap and assign tasks for Q3"),
            commit_type!("Spacewalk", "Swift Work Above Limits, Keeping All Systems Extra Safe", "Urgent hotfixes or critical production updates.", "Spacewalk(Security): Patch critical vulnerability in authentication module"),
            commit_type!("Moon Landing", "Major Leaps Over Night, New Doors and Incredible Achievements", "Completing major milestones or goals", "Moon Landing: Successfully launch beta version to select users"),
            commit_type!("First Contact", "Forge Initial Connections, Open New Territories", "Establishing initial connections or integrations", "First Contact(API): Integrate with new payment provider's API"),
            commit_type!("Interstellar Communication", "Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly", "Improving documentation or communication", "Interstellar Communication: Update wiki with troubleshooting guide for common errors"),
        ],
    );

    themes.insert(
        CommitCategory::CelestialEvents,
        vec![
            commit_type!(
                "Solar Eclipse",
                "Sun Escapes, Legacy Code Lurks",
                "Temporarily masking functionality.",
                "Solar Eclipse(Feature): Temporarily disable new onboarding flow for testing"
            ),
            commit_type!(
                "Supernova",
                "Sudden Unbelievable Performance Revolution, New Version Arrives",
                "Major, transformative change or improvement.",
                "Supernova(Architecture): Migrate to microservices architecture"
            ),
            commit_type!(
                "Meteor Shower",
                "Many Edits, Tiny Overall Result, Overhaul Routines",
                "Series of small changes or fixes.",
                "Meteor Shower: Small alignment fixes"
            ),
            commit_type!(
                "Cosmic Dawn",
                "Creating Original, Simple, Minimal Initial Draft",
                "Initial implementation of a feature.",
                "Cosmic Dawn(Search): Initial implementation of basic search functionality"
            ),
            commit_type!(
                "Solar Storm",
                "Sudden Transformations Occur Rapidly, Modifications",
                "Rapid, impactful changes.",
                "Solar Storm(Refactor): Overhaul data processing pipeline for improved performance"
            ),
            commit_type!(
                "Lunar Transit",
                "Little Update, Now Adjustments Require Testing",
                "Minor, temporary change.",
                "Lunar Transit(Config): Temporarily adjust logging level for debugging"
            ),
            commit_type!(
                "Perihelion",
                "Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal",
                "Significant milestone or feature completion.",
                "Perihelion: Successfully complete user acceptance testing for new dashboard"
            ),
            commit_type!(
                "Aphelion",
                "Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles",
                "Refactor, dependency update, or architecture change.",
                "Aphelion: Upgrade to React 18 and refactor components"
            ),
        ],
    );

    themes.insert(
        CommitCategory::CelestialObjects,
        vec![
            commit_type!(
                "White Dwarf",
                "Writing, Improving, Detailed Documentation For All",
                "Improving code comments or documentation",
                "White Dwarf(API): Add detailed documentation for new endpoints"
            ),
            commit_type!(
                "Red Giant",
                "Refactoring, Enhancing, Growing, Increasing, Adding New Things",
                "Expanding a feature or functionality",
                "Red Giant(Payments): Add support for Apple Pay and Google Pay"
            ),
            commit_type!(
                "Neutron Star",
                "New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution",
                "Optimizing code for performance",
                "Neutron Star(Search): Optimize search algorithm for faster results"
            ),
            commit_type!(
                "Binary Star",
                "Bringing In New And Revised, Yielding Integrated Results",
                "Merging features or components",
                "Binary Star: Merge user authentication and authorization modules"
            ),
            commit_type!(
                "Brown Dwarf",
                "Barely Developed, Requires Work, Ongoing Development For Future",
                "Undeveloped feature with potential",
                "Brown Dwarf(Social): Initial prototype for social sharing feature"
            ),
            commit_type!(
                "Quark Star",
                "Questionable, Unstable, Anticipated Results, Risky, Keen Experiment",
                "Experimental or speculative change",
                "Quark Star(AI): Experiment with integrating GPT-3 for content generation"
            ),
            commit_type!(
                "Rogue Planet",
                "Refactoring Or Generating Operations, Unique Path, Leaping Ahead",
                "Independent change unrelated to the main codebase",
                "Rogue Planet: Create standalone script for data migration"
            ),
            commit_type!(
                "Stellar Nursery",
                "Starting To Enhance, Laying Layers, Launching New Requirements",
                "Creating new components",
                "Stellar Nursery(UI): Add new component library for design system"
            ),
            commit_type!(
                "Planetary Nebula",
                "Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed",
                "Removal or deprecation of a component",
                "Planetary Nebula: Remove legacy image carousel component"
            ),
            commit_type!(
                "Globular Cluster",
                "Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related",
                "Collection of related changes",
                "Globular Cluster(Refactor): Refactor multiple API endpoints for consistency"
            ),
            commit_type!(
                "Void",
                "Vanished, Obliterated, Irrelevant, Deleted",
                "Removal of a module, component, or feature",
                "Void: Remove unused user settings module"
            ),
        ],
    );

    themes.insert(
        CommitCategory::AstronomicalConcepts,
        vec![
            commit_type!("Gravity", "Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding", "Resolving merge conflicts or dependencies", "Gravity: Resolve merge conflicts in feature/new-navigation branch"),
            commit_type!("Dark Matter", "Debugging And Resolving Mysterious Attributes, Tricky issues Removed", "Fixing unknown or mysterious bugs", "Dark Matter: Fix intermittent crash on user login"),
            commit_type!("Time Dilation", "Time Is Dilated, Improvements Leverage Agility, Time-Saving", "Improving code performance or reducing execution time.", "Time Dilation(Backend): Optimize image processing algorithm for faster response"),
            commit_type!("Spacetime", "Scheduling, Planning, Adjusting Calendar Events, Coordinating Time", "Changes to date, time, or scheduling", "Spacetime(API): Fix timezone handling for event timestamps"),
            commit_type!("Gravitational Lensing", "Gravity Redirects Light, Altering Information Pathways", "Altering data or information flow", "Gravitational Lensing(Data): Refactor data pipeline for improved throughput"),
            commit_type!("Cosmic String", "Connecting Our Sections, Merging Together, Interlinking New Groups", "Connecting code parts", "Cosmic String(API): Connect user service with authentication middleware"),
            commit_type!("Quantum Fluctuation", "Quick Unpredictable Adjustments, Noticed Tiny Unexpected Modification", "Small, random change", "Quantum Fluctuation: Fix typo in error message"),
            commit_type!("Hawking Radiation", "Hastily And Willingly Killing Redundancies, Ageing Dead-ends, Tidying In Order, Obliterating Noise", "Removing technical debt", "Hawking Radiation: Remove unused CSS classes and refactor styles"),
            commit_type!("Quantum Entanglement", "Quantum Effects Never Tangled, Greater Efficiency, Linked Adjustments", "Establishing close relationships between code parts", "Quantum Entanglement(API): Tightly couple user profile and order history endpoints"),
            commit_type!("Gravitational Redshift", "Gravity Reduces Efficiency, Degraded Speed, Shift Happens", "Slowing down or reducing code performance", "Gravitational Redshift(UI): Disable unnecessary animations for low-end devices"),
        ],
    );

    themes.insert(
        CommitCategory::SpaceExploration,
        vec![
            commit_type!("Space Probe", "Surveying, Planning, Analysing, Checking Every Nook", "Testing new features or technologies", "Space Probe(AI): Experiment with ChatGPT integration for customer support"),
            commit_type!("Space Station", "Setting Up The Area, Testing In Orbit, Optimising New", "Creating or improving environments", "Space Station(DevOps): Set up new development environment with Docker"),
            commit_type!("Rocket Launch", "Releasing Our Code, Keenly Entering The Production", "Deploying to production", "Rocket Launch(v1.5): Deploy new version to production with enhanced security features"),
            commit_type!("Spacewalk", "Swift Patches And Lookout Work, Keeping Systems Extra safe", "Urgent production hotfixes", "Spacewalk(Database): Fix critical database connection issue causing downtime"),
            commit_type!("Space Elevator", "Streamlined Access, Providing Easy Vertical On boarding, Lifting Entries", "Making code base more accessible", "Space Elevator: Refactor README for onboarding"),
        ],
    );

    themes
}

pub struct Log {
    pub author: String,
    pub message: String,
    pub at: String,
    pub signature: String,
    pub changes: Vec<(String, FileChange)>,
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self.author.split("<").collect::<Vec<&str>>();
        let author = x[0].trim().to_string();
        writeln!(f, "\n{author} at {} ({})\n", self.at, self.signature)?;
        writeln!(f, "\n{}\n", self.message)?;

        if !self.changes.is_empty() {
            let mut root = Tree::default();
            for (path, change) in &self.changes {
                let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
                insert_into_tree(&mut root, &parts, change.clone());
            }
            print_tree(f, &root, "", true)?; // print from root
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct Tree {
    children: BTreeMap<String, Tree>,
    is_file: bool,
    change: Option<FileChange>,
}

fn insert_into_tree(node: &mut Tree, parts: &[&str], change: FileChange) {
    if parts.is_empty() {
        return;
    }
    let first = parts[0];
    let child = node
        .children
        .entry(first.to_string())
        .or_insert_with(Tree::default);
    if parts.len() == 1 {
        child.is_file = true;
        child.change = Some(change);
    } else {
        insert_into_tree(child, &parts[1..], change);
    }
}

fn print_tree(f: &mut Formatter<'_>, node: &Tree, prefix: &str, is_root: bool) -> std::fmt::Result {
    // For root, we don't print a name, only its children
    let len = node.children.len();
    let mut i = 0usize;
    for (name, child) in &node.children {
        i += 1;
        let is_last = i == len;
        let connector = if is_last { "└──" } else { "├──" };
        if child.is_file {
            // Affiche le marqueur et les compteurs
            let marker = match &child.change {
                Some(FileChange::Added { added, mode }) => {
                    let m = mode.map(|v| crate::vcs::format_mode(v)).unwrap_or_default();
                    format!("{m} + {added}")
                }
                Some(FileChange::Deleted { deleted, mode }) => {
                    let m = mode.map(|v| crate::vcs::format_mode(v)).unwrap_or_default();
                    format!("{m} - {deleted}")
                }
                Some(FileChange::Modified {
                    added,
                    deleted,
                    mode,
                }) => {
                    let m = mode.map(|v| crate::vcs::format_mode(v)).unwrap_or_default();
                    format!("{m} ~ +{added} -{deleted}")
                }
                _ => String::new(),
            };
            writeln!(f, "{prefix}{connector} {marker} {name}")?;
        } else {
            writeln!(f, "{prefix}{connector} {name}")?;
        }
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        print_tree(f, child, &new_prefix, false)?;
    }
    if is_root && len == 0 {
        // nothing to print
    }
    Ok(())
}
#[derive(Debug, Clone)]
pub enum FileChange {
    Added {
        added: usize,
        mode: Option<i64>,
    },
    Deleted {
        deleted: usize,
        mode: Option<i64>,
    },
    Modified {
        added: usize,
        deleted: usize,
        mode: Option<i64>,
    },
}

pub fn author() -> String {
    use crate::db::connect_lys;
    use std::path::Path;

    // 1. On tente de lire l'identité souveraine dans la config SQLite
    if let Ok(conn) = connect_lys(Path::new(".")) {
        let mut stmt = conn
            .prepare("SELECT value FROM config WHERE key = 'author'")
            .unwrap();
        if let Ok(sqlite::State::Row) = stmt.next() {
            if let Ok(val) = stmt.read::<String, _>(0) {
                if !val.trim().is_empty() {
                    return val;
                }
            }
        }
    }

    // 2. Fallback : Identité système originale si la DB est vide
    let u = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    #[cfg(unix)]
    {
        // Sur Unix, on tente de récupérer le "Real Name" (GECOS)
        if let Ok(Some(user)) = User::from_name(u.as_str()) {
            let gecos = user.gecos.to_string_lossy().to_string();
            if !gecos.is_empty() {
                return gecos;
            }
        }
    }
    u
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Commit {
    pub category: CommitCategory,
    pub types: CommitType,
    pub os: String,
    pub os_release: String,
    pub os_version: String,
    pub os_domain: String,
    pub machine: String,
    pub arch: String,
    pub summary: String,
    pub why: String,
    pub who: String,
    pub src: String,
    pub how: String,
    pub when: String,
    pub what: String,
    pub where_path: Vec<String>,
    pub outcome: String,
    pub impact: String,
    pub breaking_changes: String,
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self.types.to_string();
        let t = x.split(":");
        writeln!(
            f,
            "\n{}: {}\n",
            t.last().expect("").trim().to_string(),
            self.summary
        )?;
        writeln!(f, "What?\n\n{}\n", self.what)?;
        writeln!(f, "Why?\n\n{}\n", self.why)?;
        writeln!(f, "How?\n\n{}\n", self.how)?;
        writeln!(f, "Breaking Changes?\n\n{}\n", self.breaking_changes)?;
        writeln!(
            f,
            "Commited on {} {} {}\n",
            self.os, self.os_release, self.arch
        )?;
        Ok(())
    }
}
impl Commit {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// # Errors
    ///
    /// Bad user or cancel by user
    ///
    pub fn confirm(&mut self) -> InquireResult<&mut Self> {
        println!("{self}");
        if Confirm::new("Confirm Commit?")
            .with_default(true)
            .prompt()?
        {
            Ok(self)
        } else if Confirm::new("Change Commit Message?")
            .with_default(false)
            .prompt()?
        {
            self.commit()
        } else {
            Err(InquireError::from(Error::other("commit aborted")))
        }
    }
    ///
    /// Commit the changes to the repository
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn commit(&mut self) -> InquireResult<&mut Self> {
        if run_hooks().is_ok() {
            self.ask_category()?
                .ask_types()?
                .ask_summary()?
                .ask_why()?
                .ask_what()?
                .ask_how()?
                .ask_benefits()?
                .ask_breaking()?
                .human_and_system()?
                .confirm()
        } else {
            Err(InquireError::OperationCanceled)
        }
    }

    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_types(&mut self) -> InquireResult<&mut Self> {
        let x = get_space_themes();
        let y = x.get(&self.category).expect("a");
        self.types = Select::new("commit types", y.clone()).prompt()?;
        Ok(self)
    }

    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_summary(&mut self) -> InquireResult<&mut Self> {
        self.summary.clear();
        while self.summary.is_empty() {
            self.summary.clear();
            self.summary
                .push_str(Text::new("Commit summary:").prompt()?.as_str());
        }
        if self.summary.is_empty() {
            return Err(InquireError::from(Error::other("bad summary")));
        }
        Ok(self)
    }
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_category(&mut self) -> InquireResult<&mut Self> {
        self.category = Select::new("", CommitCategory::all()).prompt()?;
        Ok(self)
    }

    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_breaking(&mut self) -> InquireResult<&mut Self> {
        self.breaking_changes.clear();
        while self.breaking_changes.is_empty() {
            self.breaking_changes.clear();
            self.breaking_changes
                .push_str(Text::new("Breaking Changes?").prompt()?.as_str());
        }
        if self.breaking_changes.is_empty() {
            return Err(InquireError::from(Error::other("bad changes")));
        }
        Ok(self)
    }

    ///
    /// Why are you making these changes?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_what(&mut self) -> InquireResult<&mut Self> {
        self.what.clear();
        while self.what.is_empty() {
            self.what.clear();
            self.what
                .push_str(Editor::new("What changes?").prompt()?.as_str());
        }
        if self.what.is_empty() {
            return Err(InquireError::from(Error::other("bad what")));
        }
        Ok(self)
    }

    ///
    /// Why are you making these changes?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_why(&mut self) -> InquireResult<&mut Self> {
        self.why.clear();
        while self.why.is_empty() {
            self.why.clear();
            self.why
                .push_str(Editor::new(WHY_PROMPT).prompt()?.as_str());
        }
        if self.why.is_empty() {
            return Err(InquireError::from(Error::other("bad why")));
        }
        Ok(self)
    }

    ///
    /// Why are you making these changes?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_how(&mut self) -> InquireResult<&mut Self> {
        self.how.clear();
        while self.how.is_empty() {
            self.how.clear();
            self.how
                .push_str(Editor::new(HOW_PROMPT).prompt()?.as_str());
        }
        if self.why.is_empty() {
            return Err(InquireError::from(Error::other("bad why")));
        }
        Ok(self)
    }

    pub fn human_and_system(&mut self) -> InquireResult<&mut Self> {
        self.os.clear();
        self.os_version.clear();
        self.os_release.clear();
        self.os_domain.clear();
        self.machine.clear();
        self.arch.clear();
        self.who.clear();
        self.when.clear();
        self.arch.push_str(ARCH);
        self.when.push_str(
            Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .as_str(),
        );
        #[cfg(unix)]
        {
            let o = uname().expect("failed");
            self.os
                .push_str(o.sysname().to_str().expect("").to_string().as_str());
            self.machine
                .push_str(o.machine().to_str().expect("").to_string().as_str());
            self.os_release
                .push_str(o.release().to_str().expect("").to_string().as_str());
            self.os_version
                .push_str(o.version().to_str().expect("").to_string().as_str());
            self.os_domain
                .push_str(o.nodename().to_str().expect("").to_string().as_str());
        }
        #[cfg(windows)]
        {
            let os_name = std::env::consts::OS;
            let os_release = std::env::var("OS").unwrap_or_else(|_| "Windows".to_string());
            let machine = std::env::var("COMPUTERNAME").unwrap_or_default();
            let domain = std::env::var("USERDOMAIN").unwrap_or_default();
            self.os.push_str(os_name);
            self.machine.push_str(machine.as_str());
            self.os_release.push_str(os_release.as_str());
            self.os_version.push_str(os_release.as_str());
            self.os_domain.push_str(domain.as_str());
        }
        self.who.push_str(author().as_str());
        Ok(self)
    }

    ///
    /// What code resolve
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_benefits(&mut self) -> InquireResult<&mut Self> {
        self.outcome.clear();
        while self.outcome.is_empty() {
            self.outcome.clear();
            self.outcome
                .push_str(Editor::new(OUTCOME_PROMPT).prompt()?.as_str());
        }
        if self.outcome.is_empty() {
            return Err(InquireError::from(Error::other("bad benefits")));
        }
        Ok(self)
    }
}
