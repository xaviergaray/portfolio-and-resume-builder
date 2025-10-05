use std::fs::read_to_string;

pub trait ResumeItem {
    fn get_latex(&self) -> String;

    fn from_json(json: &serde_json::Value) -> Self where Self: Sized;

    fn from_json_array(json: &serde_json::Value) -> Vec<Self>  where Self: Sized {
        json.as_array()
            .expect("Expected array")
            .iter()
            .map(Self::from_json)
            .collect()
    }
}

pub struct ResumeSection {
    title: String,
    items: Vec<Box<dyn ResumeItem>>,
}

impl ResumeSection {
    pub fn new(title: String, items: Vec<Box<dyn ResumeItem>>) -> Self {
        Self { title, items }
    }

    pub fn new_sections(sections: Vec<(String, Vec<Box<dyn ResumeItem>>)>) -> Vec<Self> {
        sections.into_iter()
            .map(|(title, items)| Self::new(title, items))
            .collect()
    }

    pub fn get_latex(&self) -> String {
        let mut latex = "".to_owned();
        if !self.items.is_empty() {
            latex = format!("\n\\header{{{}}}\n", self.title);

            self.items.iter().for_each(|e| {
                latex += e.get_latex().as_str();
            })
        }

        latex + "\n"
    }
}

pub struct Certification {
    title: String,
    description: String,
}

impl ResumeItem for Certification {
    fn get_latex(&self) -> String {
        format!(r#"
            {{\textbf{{{}}}}}\\
            {}\\
            \vspace*{{2mm}}
            "#, self.title, self.description)
    }

    fn from_json(json: &serde_json::Value) -> Self {
        let title = json["title"].as_str().expect("Failed to parse details").to_string();
        let description = json["description"].as_str().expect("Failed to parse details").to_string();

        Self {
            title,
            description,
        }
    }
}

pub struct Education {
    school: String,
    major: String,
    location: String,
    comments: String,
    dates: String,
}

impl ResumeItem for Education {
    fn get_latex(&self) -> String {
        let comments = match self.comments.as_str() {
            "" => {"".to_owned()},
            _ => {"; ".to_owned() + &self.comments}
        };

        format!(r#"
            \textbf{{{}}} \hfill {}\\
            {}{} \hfill {}\\
            \vspace{{2mm}}
            "#, self.major, self.location, self.school, comments, self.dates)
    }

    fn from_json(json: &serde_json::Value) -> Self {
        let school = json["school"].as_str().expect("Failed to parse details").to_string();
        let major = json["major"].as_str().expect("Failed to parse details").to_string();
        let comments = json["comments"].as_str().unwrap_or("").to_string();
        let location = json["location"].as_str().expect("Failed to parse details").to_string();
        let dates = json["dates"].as_str().expect("Failed to parse details").to_string();

        Self {
            school,
            major,
            location,
            comments,
            dates,
        }
    }
}

pub struct Project {
    name: String,
    skills: Vec<String>,
    summary: Vec<String>,
}

impl ResumeItem for Project {
    fn get_latex(&self) -> String {
        let mut latex = format!(r#"
            {{\textbf{{{}}}}}\\
            \textit{{{}}}
            \vspace{{-1mm}}
            \begin{{itemize}} \itemsep 1pt
            "#, self.name, self.skills.join(", "));

        for detail in &self.summary {
            latex.push_str(&format!(r#"
                    \item {detail}
                "#));
        }

        latex += r#"
            \end{itemize}
            \vspace*{2mm}
            "#;

        latex
    }

    fn from_json(json: &serde_json::Value) -> Self {
        let name = json["name"].as_str().expect("Failed to parse details").to_string();
        let skills = json["skills"]
            .as_array()
            .expect("Failed to parse details")
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();
        let summary = json["summary"]
            .as_array()
            .expect("Failed to parse details")
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();

        Self {
            name,
            skills,
            summary,
        }
    }
}

pub struct WorkExperience {
    title: String,
    company: String,
    location: String,
    dates: String,
    details: Vec<String>,
}

impl ResumeItem for WorkExperience {
    fn get_latex(&self) -> String {
        let mut latex = format!(r#"
                \textbf{{{}}} \hfill {}\\
                \textit{{{}}} \hfill {}\\
                \vspace{{-1mm}}
                \begin{{itemize}} \itemsep 1pt
                "#, self.company, self.location, self.title, self.dates);

        for detail in &self.details {
            let detail = detail
                .as_str()
                .replace("<strong>", r#"\textbf{"#)
                .replace("</strong>", r#"}"#)
                .replace("%", r#"\%"#);

            latex.push_str(&format!(r#"
                \item {detail}"#))
        }

        latex += r#"\end{itemize}"#;

        latex
    }

    fn from_json(json: &serde_json::Value) -> Self {
        let company = json["company"].as_str().expect("Failed to parse details").to_string();
        let location = json["location"].as_str().expect("Failed to parse details").to_string();
        let title = json["title"].as_str().expect("Failed to parse details").to_string();
        let dates = json["date_range"].as_str().expect("Failed to parse details").to_string();
        let details = json["details"]
            .as_array()
            .expect("Failed to parse details")
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();

        Self {
            company,
            location,
            title,
            dates,
            details
        }
    }
}

pub struct Resume {
    sections: Vec<ResumeSection>,
    title: String,
    subtitle: Vec<String>,
}

impl Resume {
    fn build_latex_header(&self) -> String {
        let mut header = "\n\\vspace*{-10pt}\n\\begin{center}".to_owned();
        header += &format!("{{\\Huge \\scshape {{{}}}}}\\\\\n", self.title);
        header += &self.subtitle.join(r" $\cdot$ ");
        header += "\\\\\n\\end{center}";

        header
    }

    pub fn build_latex(&self) -> String {
        let mut latex = r#"
    \documentclass[a4paper]{article}
    \usepackage{fullpage}
    \usepackage{amsmath}
    \usepackage{amssymb}
    \usepackage{textcomp}
    \usepackage{tabularx}
    \usepackage[utf8]{inputenc}
    \usepackage[T1]{fontenc}
    \textheight=10in
    \pagestyle{empty}
    \raggedright
    \usepackage[left=0.8in,right=0.8in,bottom=0.8in,top=0.8in]{geometry}
    \def\bull{\vrule height 0.8ex width .7ex depth -.1ex }

    \newcommand{\area} [2] {
        \vspace*{-9pt}
        \begin{verse}
            \textbf{#1}   #2
        \end{verse}
    }

    \newcommand{\lineunder} {
        \vspace*{-8pt} \\
        \hspace*{-18pt} \hrulefill \\
    }

    \newcommand{\header} [1] {
        {\hspace*{-18pt}\vspace*{6pt} \textsc{#1}}
        \vspace*{-6pt} \lineunder
    }

    \newcommand{\employer} [3] {
        { \textbf{#1} (#2)\\ \underline{\textbf{\emph{#3}}}\\  }
    }

    \newcommand{\contact} [3] {
        \vspace*{-10pt}
        \begin{center}
            {\Huge \scshape {#1}}\\
            #2 \\ #3
        \end{center}
        \vspace*{-8pt}
    }

    \newenvironment{achievements}{
        \begin{list}
            {$\bullet$}{\topsep 0pt \itemsep -2pt}}{\vspace*{4pt}
        \end{list}
    }

    \newcommand{\schoolwithcourses} [4] {
        \textbf{#1} #2 $\bullet$ #3\\
        #4 \\
        \vspace*{5pt}
    }

    \newcommand{\school} [4] {
        \textbf{#1} #2 $\bullet$ #3\\
        #4 \\
    }

        \begin{document}
    \vspace*{-40pt}
    "#.to_owned();
        
        latex += &self.build_latex_header();

        for section in &self.sections {
            latex += section.get_latex().as_str();
        }

        let footer = r#"\end{document}"#.to_owned();
        
        latex += &footer;

        latex
    }

    pub fn new(title: String, subtitle: Vec<String>, sections: Vec<ResumeSection>) -> Self {
        Self {
            title,
            subtitle,
            sections,
        }
    }
    
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}

pub fn read_json(path: &str) -> serde_json::Value {
    let json_str = read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json_str).expect("Failed to parse JSON")
}