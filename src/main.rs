use std::fs;
use std::fs::read_to_string;
use std::path::Path;
use serde_json::Value;

fn read_json(path: &str) -> Value{
    let json_str = read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&json_str).expect("Failed to parse JSON")
}

fn build_preamble() -> String {
    let preamble = r#"
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
    "#;

    preamble.to_owned()
}

fn build_header() -> String {
    let header = r#"
    \vspace*{-10pt}
    \begin{center}
        {\Huge \scshape {Xavier Garay}}\\
        xaviergaray0010@gmail.com $\cdot$ (908) 528-4161 $\cdot$ DoD TS/SCI Security Clearance\\
    \end{center}
    "#;

    header.to_owned()
}

fn build_work_experience() -> String {
    let mut work_experience = r#"
    \header{Work Experience}
    \vspace{1mm}
    "#.to_owned();

    let experiences: Value = read_json("input/data/content/Work-Experience.json");

    if let Some(experiences) = experiences.as_array() {
        for experience in experiences {
            let company = experience["company"].as_str().expect("Failed to parse details");
            let location = experience["location"].as_str().expect("Failed to parse details");
            let title = experience["title"].as_str().expect("Failed to parse details");
            let date_range = experience["date_range"].as_str().expect("Failed to parse details");
            let details = experience["details"].as_array().expect("Failed to parse details");

            let mut tmp = format!(r#"
                \textbf{{{company}}} \hfill {location}\\
                \textit{{{title}}} \hfill {date_range}\\
                \vspace{{-1mm}}
                \begin{{itemize}} \itemsep 1pt
                "#);

            for detail in details {
                let detail = detail
                    .as_str()
                    .expect("Failed to parse details")
                    .replace("<strong>", r#"\textbf{"#)
                    .replace("</strong>", r#"}"#)
                    .replace("%", r#"\%"#);

                tmp.push_str(&format!(r#"
                \item {detail}"#))
            }

            tmp += r#"\end{itemize}"#;

            work_experience += &tmp;
        }
    };

    work_experience
}

fn build_education() -> String {
    let mut education = r#"
    \header{Education}
    "#.to_owned();

    let degrees = read_json("input/data/content/Education.json");

    if let Some(degrees) = degrees.as_array() {
        for degree in degrees {
            let school = degree["school"].as_str().expect("Failed to parse details");
            let major = degree["major"].as_str().expect("Failed to parse details");
            let comments = degree["comments"].as_str().unwrap_or("");
            let location = degree["location"].as_str().expect("Failed to parse details");
            let dates = degree["dates"].as_str().expect("Failed to parse details");

            let comments = match comments {
                "" => {"".to_owned()},
                _ => {"; ".to_owned() + comments}
            };

            let tmp = format!(r#"
            \textbf{{{major}}} \hfill {location}\\
            {school}{comments} \hfill {dates}\\
            \vspace{{2mm}}
            "#);

            education.push_str(&tmp);
        }
    }

    education
}

fn build_certifications() -> String {
    let mut certifications = r#"
    \header{Certifications}
    "#.to_owned();

    let certs = read_json("input/data/content/Certifications.json");

    if let Some(certs) = certs.as_array() {
        for cert in certs {
            let title = cert["title"].as_str().expect("Failed to parse details");
            let description = cert["description"].as_str().expect("Failed to parse details");

            let tmp = format!(r#"
            {{\textbf{{{title}}}}}\\
            {description}\\
            \vspace*{{2mm}}
            "#);

            certifications.push_str(&tmp);
        }
    }

    certifications
}

fn build_projects() -> String {
    let mut projects = r#"
    \header{Projects}
    "#.to_owned();

    let projs_json = read_json("input/data/content/Projects.json");
    let skills_json = read_json("input/data/content/Skills.json");

    let mut skill_map = std::collections::HashMap::new();
    if let Some(skill_list) = skills_json.as_array() {
        for skill in skill_list {
            if let Some(slug) = skill["slug"].as_str() {
                skill_map.insert(slug.to_string(), skill);
            }
        }
    }

    if let Some(projs) = projs_json.as_array() {
        for proj in projs {
            let name = proj["name"].as_str().expect("Failed to parse details");
            let skills = proj["skills"]
                .as_array()
                .expect("Failed to parse details")
                .iter()
                .filter_map(|val| val["skill_slug"].as_str())
                .filter_map(|slug| skill_map.get(slug))
                .filter_map(|skill| skill["skill"].as_str())
                .collect::<Vec<&str>>()
                .join(", ");

            let summary = proj["summary"].as_array().expect("Failed to parse details");

            let mut tmp = format!(r#"
            {{\textbf{{{name}}}}}\\
            \textit{{{skills}}}
            \vspace{{-1mm}}
            \begin{{itemize}} \itemsep 1pt
            "#);

            for detail in summary {
                let detail = detail.as_str().expect("Failed to parse skill");

                tmp.push_str(&format!(r#"
                    \item {detail}
                "#));
            }

            tmp += r#"
            \end{itemize}
            \vspace*{2mm}
            "#;

            projects.push_str(&tmp);
        }
    }

    projects
}

fn build_resume_latex() -> String{
    let footer = r#"\end{document}"#;

    build_preamble()
        + &build_header()
        + &build_work_experience()
        + &build_education()
        + &build_certifications()
        + &build_projects()
        + footer
}

fn write_and_compile_latex(output_path: &str, tex_filename: &str) {
    let latex_code = build_resume_latex();

    let output_dir = Path::new(output_path);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("failed to create output directory");
    }

    fs::write(tex_filename, latex_code).expect("failed to write latex file");

    let status = std::process::Command::new("pdflatex")
        .arg("-interaction=nonstopmode")
        .arg("-output-directory")
        .arg(output_path)
        .arg(tex_filename)
        .status().expect("failed to execute pdflatex");

    if status.success() {
        println!("PDF successfully compiled to {}/{}.pdf", output_path, Path::new(tex_filename).file_stem().unwrap().to_str().unwrap());
    } else {
        eprintln!("Failed to compile LaTeX.");
    }
}

fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("failed to create parent directory");
        }
    }

    fs::copy(src, dst).expect(&format!("failed to copy file from {} to {}", src.display(), dst.display()));
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !dst.exists() {
        fs::create_dir_all(dst).expect("failed to create destination directory");
    }

    for entry in fs::read_dir(src).expect("failed to read directory") {
        let entry = entry.expect("failed to read directory entry");
        let file_type = entry.file_type().expect("failed to get file type");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(src_path, dst_path);
        } else {
            copy_file(&src_path, &dst_path);
        }
    }
}

fn main() {
    write_and_compile_latex("output", "output/GarayXavierResume.tex");
    copy_dir_recursive("input/public", "NextApp/public");
    copy_dir_recursive("input/data", "NextApp/src/data");
    copy_file("output/GarayXavierResume.pdf", "NextApp/public/files/GarayXavierResume.pdf");
}