import skills from "@/data/content/Skills.json";
import experiences from "@/data/content/Work-Experience.json";
import projects from "@/data/content/Projects.json";
import { notFound } from "next/navigation";
import React from "react";
import Link from "next/link";

export default async function Page({ params, }: { params: Promise<{ slug: string }> }) {
    const { slug } = await params;
    const skill = skills.find((s) => s.slug === slug);

    if (!skill) return notFound();

    const relatedExperiences = experiences.flatMap((exp) => {
        const match = exp.skills.find((s) => s.skill_slugs.includes(skill.slug));
        return match
            ? [{
                ...exp,
                matchingDescription: match.description,
            }]
            : [];
    });

    const relatedProjects = projects.flatMap((proj) => {
        const match = proj.skills.find((s) => s.skill_slug === skill.slug);
        return match
            ? [{
                ...proj,
                matchingDescription: match.description,
            }]
            : [];
    });

    return (
        <div className="flex flex-col items-center w-[80vw] bg-[var(--backdrop)] rounded-lg shadow p-10 gap-8">
            <h1 className="text-4xl">{skill.skill}</h1>
            <ul className="list-disc space-y-2 w-full">
                {skill.details.map(detail => (
                    <li key={detail} className="text-[var(--maintext2)] [&_a]:text-blue-500 [&_a]:underline [&_a]:hover:text-blue-700" dangerouslySetInnerHTML={{__html: detail}}></li>
                ))}
            </ul>

            {relatedExperiences.length > 0 && (
                <div className="flex flex-col items-center w-full gap-4">
                    <h1 className="text-2xl">Related Work Experience</h1>
                    <div className="flex flex-row flex-wrap justify-center gap-6 w-full">
                        {relatedExperiences.map((exp) => (
                            <Link
                                key={exp.slug}
                                href={`/work-experience/${exp.slug}`}
                                className="custom_button w-[auto] h-[auto] p-5"
                            >
                                <h2 className="text-lg">{exp.title}</h2>
                                <p className="text-[var(--maintext2)]">{exp.company}</p>
                            </Link>
                        ))}
                    </div>
                </div>
            )}

            {relatedProjects.length > 0 && (
                <div className="flex flex-col items-center w-full gap-4">
                    <h1 className="text-2xl">Related Projects</h1>
                    <div className="flex flex-row flex-wrap justify-center gap-6 w-full">
                        {relatedProjects.map((proj) => (
                            <Link
                                key={proj.slug}
                                href={`/projects/${proj.slug}`}
                                className="custom_button w-[auto] h-[auto] p-5"
                            >
                                <h2 className="text-lg">{proj.name}</h2>
                            </Link>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}


export function generateStaticParams() {
    return skills.map((s) => ({
        slug: s.slug,
    }));
}