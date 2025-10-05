use crate::{read_json, structures};

use std::fs;
use std::path::Path;
use serde_json::{Value, Map};
use eframe::egui::{self, Ui};
use structures::{Resume, WorkExperience, Project, Certification, Education, ResumeItem, ResumeSection};

/// Recursive tree node
#[derive(Debug, Clone)]
struct TreeNode {
    label: String,
    checked: bool,
    children: Vec<TreeNode>,
    value: Option<Value>, // store original JSON value for leaves
}


impl TreeNode {
    fn from_json(key: &str, value: &Value) -> Self {
        match value {
            Value::Object(map) => {
                let children = map
                    .iter()
                    .map(|(k, v)| TreeNode::from_json(k, v))
                    .collect();
                TreeNode {
                    label: key.to_string(),
                    checked: true,
                    children,
                    value: None,
                }
            }
            Value::Array(arr) => {
                let children = arr
                    .iter()
                    .enumerate()
                    .map(|(i, v)| TreeNode::from_json(&format!("{}[{}]", key, i), v))
                    .collect();
                TreeNode {
                    label: key.to_string(),
                    checked: true,
                    children,
                    value: None,
                }
            }
            _ => TreeNode {
                label: format!("{}: {}", key, value),
                checked: true,
                children: Vec::new(),
                value: Some(value.clone()), // keep actual JSON leaf
            },
        }
    }

    /// Recursive UI renderer
    fn show(&mut self, ui: &mut Ui) {
        let mut parent_checked = self.checked;
        let response = ui.checkbox(&mut parent_checked, &self.label);
        if response.changed() {
            self.set_checked(parent_checked);
        }

        if !self.children.is_empty() {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(&self.label),
                false,
            )
                .show_header(ui, |ui| {})
                .body(|ui| {
                    ui.indent(&self.label, |ui| {
                        for child in &mut self.children {
                            child.show(ui);
                        }
                    });
                });
        }
    }

    /// Recursively set all children checked/unchecked
    fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        for child in &mut self.children {
            child.set_checked(checked);
        }
    }
}

/// Section = filename + parsed tree
struct TmpSection {
    filename: String,
    root: TreeNode,
}

struct ResumeApp {
    sections: Vec<TmpSection>,
    title: String,
    subtitles: Vec<String>,
}

impl ResumeApp {
    fn load_sections() -> Vec<TmpSection> {
        let mut sections = Vec::new();
        let path = Path::new("input/data/content");

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            // Build root node directly from JSON
                            let root = TreeNode::from_json(
                                &path.file_name().unwrap().to_string_lossy(),
                                &json,
                            );

                            sections.push(TmpSection {
                                filename: path
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string(),
                                root,
                            });
                        }
                    }
                }
            }
        }

        sections
    }


    /// Recursively collect only checked nodes as JSON Values
    fn collect_checked(node: &TreeNode) -> Option<Value> {
        if !node.checked {
            return None;
        }

        if let Some(val) = &node.value {
            // Leaf → return real JSON value
            return Some(val.clone());
        }

        // If it has children, decide between object vs array
        if !node.children.is_empty() {
            // If children are indexed (0,1,2...), treat as array
            let is_array = node.children.iter().all(|c| {
                c.label.chars().all(|ch| ch.is_ascii_digit()) // crude array detection
                    || c.label.contains('[')                 // e.g., "skills[0]"
            });

            return if is_array {
                let mut arr = Vec::new();
                for child in &node.children {
                    if let Some(val) = Self::collect_checked(child) {
                        arr.push(val);
                    }
                }
                Some(Value::Array(arr))
            } else {
                let mut map = serde_json::Map::new();
                for child in &node.children {
                    if let Some(val) = Self::collect_checked(child) {
                        let key = child.label.split(&[':', '['][..]).next().unwrap_or(&child.label);
                        map.insert(key.to_string(), val);
                    }
                }
                Some(Value::Object(map))
            }
        }

        None
    }


    fn generate_resume(&self) -> Resume {
        // Collect sections by filename or type
        let mut work_experience: Vec<Box<dyn ResumeItem>> = Vec::new();
        let mut projects: Vec<Box<dyn ResumeItem>> = Vec::new();
        let mut education: Vec<Box<dyn ResumeItem>> = Vec::new();
        let mut certifications: Vec<Box<dyn ResumeItem>> = Vec::new();

        for section in &self.sections {
            if let Some(json) = Self::collect_checked(&section.root) {
                match section.filename.as_str() {
                    "Work-Experience.json" => {
                        work_experience.extend(WorkExperience::from_json_array(&json).into_iter().map(|e| Box::new(e) as Box<dyn ResumeItem>));
                    }
                    "Projects.json" => {
                        projects.extend(Project::from_json_array(&json).into_iter().map(|e| Box::new(e) as Box<dyn ResumeItem>));
                    }
                    "Education.json" => {
                        education.extend(Education::from_json_array(&json).into_iter().map(|e| Box::new(e) as Box<dyn ResumeItem>));
                    }
                    "Certifications.json" => {
                        certifications.extend(Certification::from_json_array(&json).into_iter().map(|e| Box::new(e) as Box<dyn ResumeItem>));
                    }
                    _ => {}
                }
            }
        }

        let sections = ResumeSection::new_sections(vec![
            ("Work Experience".to_owned(), work_experience),
            ("Education".to_owned(), education),
            ("Certifications".to_owned(), certifications),
            ("Projects".to_owned(), projects),
        ]);

        Resume::new(
            self.title.clone(),
            self.subtitles.clone(),
            sections,
        )
    }
}

impl eframe::App for ResumeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                // --- Title field ---
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.title);
                });

                ui.separator();

                // --- Subtitle fields ---
                ui.label("Subtitles:");
                let mut to_remove = None;
                for (i, subtitle) in self.subtitles.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(subtitle);
                        if ui.button("−").clicked() {
                            to_remove = Some(i);
                        }
                    });
                }
                if let Some(i) = to_remove {
                    self.subtitles.remove(i);
                }
                if ui.button("+ Add Subtitle").clicked() {
                    self.subtitles.push(String::new());
                }

                ui.separator();

                // --- JSON tree sections ---
                for section in &mut self.sections {
                    ui.heading(&section.filename);
                    section.root.show(ui);
                    ui.separator();
                }

                // --- Generate button ---
                if ui.button("Generate Resume").clicked() {
                    let resume = self.generate_resume();
                    let latex = resume.build_latex();
                    let filename = resume.get_title().replace(' ', "_");
                    let filepath = "/tmp";

                    fs::write(format!("{filepath}/{filename}.tex"), latex).expect("failed to write latex file");

                    let status = std::process::Command::new("pdflatex")
                        .arg("-interaction=nonstopmode")
                        .arg("-output-directory")
                        .arg(filepath)
                        .arg(format!("{filename}.tex"))
                        .status().expect("failed to execute pdflatex");

                    if status.success() {
                        println!("PDF successfully compiled to {filepath}/{filename}.pdf");
                    } else {
                        eprintln!("Failed to compile LaTeX.");
                    }
                }
            });
        });
    }
}

pub fn show() -> eframe::Result<()> {
    let header = read_json("input/data/content/Resume-Header.json");

    let title = header["title"].as_str().expect("Failed to parse title").to_string();
    let subtitles = header["subtitles"].as_array()
        .expect("Failed to parse subtitles")
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect();

    let app = ResumeApp {
        sections: ResumeApp::load_sections(),
        title,
        subtitles
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Resume Tree Viewer", native_options, Box::new(|_| Ok(Box::new(app))))
}
