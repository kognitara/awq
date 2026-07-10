use crate::db::connect_lys;
use crate::todo::{TodoItem, todos};
use crate::utils::{ok, run_hooks};
use chrono::Local;
use crossterm::style::Stylize;
use git2::{IndexAddOption, Repository, Signature};
use inquire::error::InquireResult;
use inquire::{Confirm, Editor, InquireError, Select, Text};
use justify::{Settings, justify};
#[cfg(unix)]
use nix::sys::utsname::uname;
#[cfg(unix)]
use nix::unistd::User;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env::consts::ARCH;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::Path;
use tabled::builder::Builder;
use tabled::settings::Style;

#[doc = "First prompt to ask the user what is the objective of the changes"]
pub const WHAT: &str = "What is the objective of the changes?";

#[doc = "First prompt to ask the user who is the objective of the changes"]
pub const WHO: &str = "";
pub const WHERE: &str = "";
pub const WHEN: &str = "";

pub const HOW: &str = "How the changes were made and what was changed?";
pub const WHY: &str =
    "Why is the objective of the changes important and what is the expected outcome?";

pub const SUBJECT_PROMPT: &str = "Summary of changes";
pub const OUTCOME_PROMPT: &str = "Outcome of changes";

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct CommitType {
    pub name: &'static str,
    pub mnemonic: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CommitCategory {
    #[default]
    CoreChanges,
    MaintenanceInfrastructure,
    ProjectEvents,
    CommunicationCollaboration,
    CelestialEvents,
    CelestialObjects,
    AstronomicalConcepts,
    SpaceExploration,
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

/// Synchronise automatiquement les changements vers Git.
/// Prend le message de commit généré par AWQ en paramètre.
pub fn sync_to_git(commit_message: &str) -> anyhow::Result<()> {
    // 1. Découvrir le dépôt Git (cherche un .git dans le dossier courant ou parents)
    // Si aucun dépôt Git n'est trouvé, on quitte silencieusement (AWQ continue sa vie).
    let repo = match Repository::discover(Path::new(".")) {
        Ok(r) => r,
        Err(_) => return Ok(()), // Pas de .git, on ne fait rien
    };

    // 2. Ajouter tous les fichiers modifiés/ajoutés/supprimés (équivalent de `git add .`)
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // 3. Écrire l'arbre (Tree) à partir de l'index
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    // 4. Récupérer l'identité de l'auteur
    // On essaie de prendre la config Git globale, sinon on met un fallback de sécurité
    let sig = repo
        .signature()
        .unwrap_or_else(|_| Signature::now("AWQ Auto-Sync", "awq@localhost").unwrap());

    // 5. Trouver le parent (HEAD actuel)
    // C'est nécessaire car le premier commit d'un dépôt n'a pas de parent
    let parent_commit = match repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };

    // Construire la liste des références aux parents pour la fonction commit
    let mut parents = Vec::new();
    if let Some(ref parent) = parent_commit {
        parents.push(parent);
    }

    // 6. Créer le commit dans Git
    repo.commit(
        Some("HEAD"),   // Met à jour la référence HEAD
        &sig,           // Auteur
        &sig,           // Committer
        commit_message, // Le message formaté de AWQ
        &tree,          // L'arbre des fichiers
        &parents,       // Les parents (0 ou 1)
    )?;

    ok("Commit added to git");
    Ok(())
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

pub enum Level {
    Low,
    Medium,
    High,
}

fn ago(timestamp: &str) -> String {
    if let Ok(parsed_time) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        let now = Local::now();
        let duration = now.signed_duration_since(parsed_time.with_timezone(&Local));
        if duration.num_seconds() < 60 {
            format!("{} seconds ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{} minutes ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{} hours ago", duration.num_hours())
        } else if duration.num_days() < 30 {
            format!("{} days ago", duration.num_days())
        } else if duration.num_days() < 365 {
            format!("{} months ago", duration.num_days() / 30)
        } else {
            format!("{} years ago", duration.num_days() / 365)
        }
    } else {
        String::new()
    }
}

fn file_stats(
    f: &mut Formatter,
    prefix: &str,
    connector: &str,
    file: &str,
    added: usize,
    deleted: usize,
) -> std::fmt::Result {
    let total = added + deleted;
    const MAX_BAR_WIDTH: usize = 40; // La largeur maximale de ta barre de stats

    // Calcul du nombre de '+' et de '-' à afficher
    let (display_added, display_deleted) = if total > MAX_BAR_WIDTH {
        let factor = MAX_BAR_WIDTH as f64 / total as f64;
        let mut a = (added as f64 * factor).round() as usize;
        let mut d = (deleted as f64 * factor).round() as usize;

        // Sécurité pour corriger les micro-erreurs d'arrondi des f64
        while a + d > MAX_BAR_WIDTH {
            if a > d {
                a -= 1;
            } else {
                d -= 1;
            }
        }

        // Sécurité visuelle : si on a des ajouts/suppressions, on affiche au moins un caractère
        if a == 0 && added > 0 {
            a = 1;
        }
        if d == 0 && deleted > 0 {
            d = 1;
        }

        (a, d)
    } else {
        // Si ça rentre largement, on garde les vraies valeurs
        (added, deleted)
    };

    writeln!(
        f,
        "{prefix}{connector} {} {} {} {}{}",
        file,
        added.to_string().white(),
        deleted.to_string().white(),
        "+".repeat(display_added).green().bold(),
        "-".repeat(display_deleted).red().bold()
    )?;

    Ok(())
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
        let when = ago(self.at.as_str());
        let when_display = if when.is_empty() {
            self.at.as_str()
        } else {
            when.as_str()
        };

        let mut tx = Builder::new();
        tx.push_record(["Author", "Date", "Signature"]);
        tx.push_record([self.author.as_str(), when_display, self.signature.as_str()]);
        writeln!(f)?;
        writeln!(f, "{}", tx.build().with(Style::modern()))?;
        writeln!(f, "{}\n", self.message.to_string().white().bold())?;
        writeln!(f, "\n.\n├── h {}", self.signature.to_string().white())?;

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
    let child = node.children.entry(first.to_string()).or_default();
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
            let marker: (String, usize, usize) = match &child.change {
                Some(FileChange::Added { added, mode }) => {
                    let m = mode.map(crate::vcs::format_mode).unwrap_or_default();
                    (
                        format!("{} {}", m.white(), name.clone().white().bold()),
                        *added,
                        0,
                    )
                }
                Some(FileChange::Deleted { deleted, mode }) => {
                    let m = mode.map(crate::vcs::format_mode).unwrap_or_default();
                    (
                        format!("{} {}", m.white(), name.clone().white().bold()),
                        0,
                        *deleted,
                    )
                }
                Some(FileChange::Modified {
                    added,
                    deleted,
                    mode,
                }) => {
                    let m = mode.map(crate::vcs::format_mode).unwrap_or_default();
                    (
                        format!("{} {}", m.white(), name.clone().white().bold()),
                        *added,
                        *deleted,
                    )
                }
                _ => (String::new(), 0, 0),
            };
            file_stats(f, prefix, connector, marker.0.as_str(), marker.1, marker.2)?;
        } else {
            writeln!(f, "{prefix}{connector} {}", name.to_string().blue().bold())?;
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
        if let Ok(sqlite::State::Row) = stmt.next()
            && let Ok(val) = stmt.read::<String, _>(0)
            && !val.trim().is_empty()
        {
            return val;
        }

        let mut stmt = conn
            .prepare("SELECT value FROM config WHERE key = 'name'")
            .unwrap();
        if let Ok(sqlite::State::Row) = stmt.next()
            && let Ok(val) = stmt.read::<String, _>(0)
            && !val.trim().is_empty()
        {
            return val;
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
    pub ticket: TodoItem,
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
pub fn format_justified(text: &str) -> String {
    justify(
        text,
        &Settings {
            width: 70,
            wcwidth: true,
            hyphenate_overflow: true,
            justify_last_line: false,
            insert_at: justify::InsertAt::Left,
            ignore_spaces: false,
            newline: "\n",
            hyphen: "-",
            separator: " ",
        },
    )
}
impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self.types.to_string();
        let tp = x.split(":");
        let mut t = Builder::new();
        t.push_record(["OS", "Release", "Arch", "Ticket", "Title", "Description"]);
        t.push_record([
            self.os.as_str(),
            self.os_release.as_str(),
            self.arch.as_str(),
            self.ticket.id.to_string().as_str(),
            self.ticket.title.to_string().as_str(),
            self.ticket.description.to_string().as_str(),
        ]);
        writeln!(f)?;
        writeln!(f, "{}", t.build().with(Style::modern()))?;
        writeln!(f)?;
        writeln!(
            f,
            "{}\n",
            format_args!(
                "{} - {}",
                tp.last().expect("").trim(),
                self.summary.trim_end()
            ),
        )?;
        writeln!(f)?;
        writeln!(f, "{}", "What?".bold())?;
        writeln!(f)?;
        writeln!(f, "{}", format_justified(self.what.as_str()).white())?;

        writeln!(f)?;
        writeln!(f, "{}", "Why?".bold())?;
        writeln!(f)?;
        writeln!(f, "{}", format_justified(self.why.as_str()).white())?;
        writeln!(f)?;
        writeln!(f, "{}", "How?".bold())?;
        writeln!(f)?;
        writeln!(f, "{}", format_justified(self.how.as_str()).white())?;
        writeln!(f)?;
        writeln!(f, "{}", "Breaking Changes?".bold())?;
        writeln!(f)?;
        writeln!(
            f,
            "{}",
            format_justified(self.breaking_changes.as_str()).white()
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
    pub fn confirm(&mut self) -> InquireResult<TodoItem> {
        println!("{self}");
        if Confirm::new("Confirm Commit?")
            .with_default(true)
            .prompt()?
        {
            Ok(self.ticket.clone())
        } else if Confirm::new("Change Commit Message?")
            .with_default(false)
            .prompt()?
        {
            self.commit()
        } else {
            Err(InquireError::from(Error::other("commit aborted")))
        }
    }
    pub fn ask_ticket(&mut self) -> InquireResult<&mut Self> {
        let x = todos(&connect_lys(Path::new(".")).expect("msg")).expect("failed to get todos");
        if x.is_empty() {
            return Err(InquireError::from(Error::other(
                "No tickets found. Please create a ticket first.",
            )));
        }
        self.ticket = Select::new("Resolves ticket:", x.clone()).prompt()?;
        Ok(self)
    }
    ///
    /// Commit the changes to the repository
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn commit(&mut self) -> InquireResult<TodoItem> {
        if run_hooks().is_ok() {
            self.ask_ticket()?
                .ask_category()?
                .ask_types()?
                .ask_summary()?
                .ask_what()?
                .ask_how()?
                .ask_benefits()?
                .ask_breaking()?
                .ask_why()?
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
                .push_str(Editor::new("Breaking Changes?").prompt()?.as_str());
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
            self.what.push_str(Editor::new(WHAT).prompt()?.as_str());
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
            self.why.push_str(Editor::new(WHY).prompt()?.as_str());
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
            self.how.push_str(Editor::new(HOW).prompt()?.as_str());
        }
        if self.how.is_empty() {
            return Err(InquireError::from(Error::other("bad how")));
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
            return Err(InquireError::from(Error::other("bad outcome")));
        }
        Ok(self)
    }
}
