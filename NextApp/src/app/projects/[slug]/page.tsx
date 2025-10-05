import projects from "@/data/content/Projects.json";
import skills from "@/data/content/Skills.json";
import Image from "next/image";
import Link from "next/link";
import React from "react";
import {notFound} from "next/navigation";

export default async function Page({ params, }: { params: Promise<{ slug: string }> }) {
    const { slug } = await params;
    const project = projects.find((p) => p.slug === slug);

    if (!project) return notFound();

    return (
        <div className="flex flex-col gap-8">
            <div className="custom_backdrop w-[80vw] flex flex-col items-center md:flex-row md:justify-between">
                <div>
                    <h1 className="text-3xl font-bold mb-4">{project.name}</h1>

                    <ul className="space-y-6">
                        {project.details.map((detail, i) => (
                            <li key={i} className="text-[var(--maintext2)] [&_a]:text-blue-500 [&_a]:underline [&_a]:hover:text-blue-700" dangerouslySetInnerHTML={{__html: detail}}/>
                        ))}
                    </ul>
                </div>

                <div className={"flex flex-col items-end md:w-1/2 justify-center"}>
                    {project.image && (
                        <Image
                            src={project.image}
                            alt={project.name}
                            width={620}
                            height={350}
                            className="object-contain border rounded-3xl "
                        />
                    ) || project.video && (
                        <iframe src={project.video} allowFullScreen className="rounded-3xl lg:h-50 lg:w-89"/>
                    )}
                </div>
            </div>

            <div className="custom_backdrop w-[80vw]">
                <h1 className="text-3xl font-bold mb-4">Relevant Skills</h1>
                <table className="table-auto w-full border-separate border-spacing-y-4">
                    <tbody>
                    {project.skills.map((projSkill, idx) => {
                        const skillObj = skills.find((s) => s.slug === projSkill.skill_slug);
                        if (!skillObj) return null;

                        return (
                            <tr key={idx} className="flex flex-col space-y-2 md:table-row md:flex-row">
                                <td className="table-cell pr-6">
                                    <Link
                                        href={`/skills/${skillObj.slug}`}
                                        className="custom_button h-[3rem] text-center whitespace-nowrap px-4"
                                    >
                                        {skillObj.skill}
                                    </Link>
                                </td>
                                <td className="table-cell align-center text-[var(--maintext2)]">
                                    {projSkill.description}
                                </td>
                            </tr>
                        );
                    })}
                    </tbody>
                </table>
            </div>
        </div>
    );
}

export function generateStaticParams() {
    return projects.map((p) => ({
        slug: p.slug,
    }));
}