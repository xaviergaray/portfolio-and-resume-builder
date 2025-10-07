use std::fs;
use std::path::Path;
use serde_json::Value;
use eframe::egui::{self, Ui};
use crate::{structures, io};
use structures::{
    Resume, WorkExperience, Project, Certification, Education, ResumeItem, ResumeSection, read_json, Skill,
};
use io::{copy_dir_recursive, copy_file, output_latex};

/// Recursive tree node
#[derive(Debug, Clone)]
struct TreeNode {
    label: String,
    checked: bool,
    children: Vec<TreeNode>,
    value: Option<Value>,
}

impl TreeNode {
    fn from_json(key: &str, value: &Value) -> Self {
        let (children, leaf) = match value {
            Value::Object(map) => (
                map.iter().map(|(k, v)| TreeNode::from_json(k, v)).collect(),
                None,
            ),
            Value::Array(arr) => (
                arr.iter()
                    .enumerate()
                    .map(|(i, v)| TreeNode::from_json(&format!("{}[{}]", key, i), v))
                    .collect(),
                None,
            ),
            _ => (vec![], Some(value.clone())),
        };

        TreeNode {
            label: key.to_string(),
            checked: true,
            children,
            value: leaf,
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        let mut checked = self.checked;
        if ui.checkbox(&mut checked, &self.label).changed() {
            self.set_checked(checked);
        }

        if !self.children.is_empty() {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(&self.label),
                false,
            )
                .show_header(ui, |_ui| {})
                .body(|ui| {
                    ui.indent(&self.label, |ui| {
                        for child in &mut self.children {
                            child.show(ui);
                        }
                    });
                });
        }
    }

    fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        for child in &mut self.children {
            child.set_checked(checked);
        }
    }
}

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
        let path = Path::new("input/data/content");
        let mut sections = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            let filename = path
                                .file_name()
                                .and_then(|f| f.to_str())
                                .unwrap_or_default()
                                .to_string();

                            sections.push(TmpSection {
                                filename: filename.clone(),
                                root: TreeNode::from_json(&filename, &json),
                            });
                        }
                    }
                }
            }
        }

        sections
    }

    fn collect_checked(node: &TreeNode) -> Option<Value> {
        if !node.checked {
            return None;
        }
        if let Some(val) = &node.value {
            return Some(val.clone());
        }
        if node.children.is_empty() {
            return None;
        }

        let is_array = node.children.iter().all(|c| c.label.chars().all(|ch| ch.is_ascii_digit()) || c.label.contains('['));

        if is_array {
            let arr: Vec<_> = node.children.iter().filter_map(Self::collect_checked).collect();
            Some(Value::Array(arr))
        } else {
            let map: serde_json::Map<_, _> = node
                .children
                .iter()
                .filter_map(|c| {
                    Self::collect_checked(c).map(|v| {
                        let key = c.label.split(&[':', '['][..]).next().unwrap_or(&c.label);
                        (key.to_string(), v)
                    })
                })
                .collect();
            Some(Value::Object(map))
        }
    }

    fn generate_resume(&self) -> Resume {
        fn box_items<T: ResumeItem + 'static>(items: Vec<T>) -> Vec<Box<dyn ResumeItem>> {
            items.into_iter().map(|i| Box::new(i) as Box<dyn ResumeItem>).collect()
        }

        let mut work_experience = Vec::new();
        let mut projects = Vec::new();
        let mut education = Vec::new();
        let mut certifications = Vec::new();

        for section in &self.sections {
            if let Some(json) = Self::collect_checked(&section.root) {
                match section.filename.as_str() {
                    "Work-Experience.json" => work_experience = box_items(WorkExperience::from_json_array(&json)),
                    "Education.json" => education = box_items(Education::from_json_array(&json)),
                    "Certifications.json" => certifications = box_items(Certification::from_json_array(&json)),
                    "Projects.json" => {
                        let mut tmp_projects = Project::from_json_array(&json);
                        let skills = Skill::from_json_array(&read_json("input/data/content/Skills.json"));
                        for project in &mut tmp_projects {
                            project.set_skill_names(&skills);
                        }
                        projects = box_items(tmp_projects);
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

        Resume::new(self.title.clone(), self.subtitles.clone(), sections)
    }
}

impl eframe::App for ResumeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.title);
                });

                ui.separator();

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

                for section in &mut self.sections {
                    ui.heading(&section.filename);
                    section.root.show(ui);
                    ui.separator();
                }

                ui.horizontal(|ui| {
                    if ui.button("Generate Resume").clicked() {
                        let resume = self.generate_resume();
                        let latex = resume.build_latex();
                        let filename = resume.get_title().replace(' ', "_");
                        output_latex(&latex, "/tmp", &filename);
                    }

                    if ui.button("Update Portfolio Website").clicked() {
                        let mut projects = Project::from_json_array(&read_json("input/data/content/Projects.json"));
                        let skills = Skill::from_json_array(&read_json("input/data/content/Skills.json"));
                        for project in &mut projects { project.set_skill_names(&skills); }

                        let full_sections = build_resume_sections_in_order(
                            WorkExperience::from_json_array(&read_json("input/data/content/Work-Experience.json")),
                            projects,
                            Education::from_json_array(&read_json("input/data/content/Education.json")),
                            Certification::from_json_array(&read_json("input/data/content/Certifications.json")),
                        );

                        let full_resume = Resume::new(self.title.clone(), self.subtitles.clone(), full_sections);
                        let filename = full_resume.get_title().replace(' ', "") + "Resume";

                        copy_dir_recursive("input/public", "NextApp/public");
                        copy_dir_recursive("input/data", "NextApp/src/data");
                        output_latex(&full_resume.build_latex(), "output", &filename);
                        copy_file(format!("output/{filename}.pdf"), format!("NextApp/public/files/{filename}.pdf"));
                    }
                });
            });
        });
    }
}

fn build_resume_sections_in_order(
    work_experience: Vec<WorkExperience>,
    project: Vec<Project>,
    education: Vec<Education>,
    certifications: Vec<Certification>,
) -> Vec<ResumeSection> {
    fn box_items<T: ResumeItem + 'static>(items: Vec<T>) -> Vec<Box<dyn ResumeItem>> {
        items.into_iter().map(|i| Box::new(i) as Box<dyn ResumeItem>).collect()
    }

    ResumeSection::new_sections(vec![
        ("Work Experience".to_owned(), box_items(work_experience)),
        ("Education".to_owned(), box_items(education)),
        ("Certifications".to_owned(), box_items(certifications)),
        ("Projects".to_owned(), box_items(project)),
    ])
}

pub fn show() -> eframe::Result<()> {
    let header = read_json("input/data/content/Resume-Header.json");

    let title = header["title"].as_str().unwrap_or_default().to_string();
    let subtitles = header["subtitles"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect();

    let app = ResumeApp {
        sections: ResumeApp::load_sections(),
        title,
        subtitles,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Resume Tree Viewer", native_options, Box::new(|_| Ok(Box::new(app))))
}
