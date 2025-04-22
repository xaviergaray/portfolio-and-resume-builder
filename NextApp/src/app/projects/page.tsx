import Link from "next/link";
import projects from "@/data/content/Projects.json";
import skills from "@/data/content/Skills.json";
import Image from "next/image";
import React from "react";

export default function Projects() {
    return (
        <>
            <h1 className="custom_backdrop text-3xl text-center mb-10">Select a project to learn more</h1>
            <div className="flex flex-col gap-8 items-center">
                {projects.map((project, idx) => (
                    <Link
                        key={project.slug}
                        href={`/projects/${project.slug}`}
                        className={`custom_backdrop custom_backdrop_link flex flex-col w-full gap-10 items-center md:flex-row ${idx % 2 === 0 ? "md:flex-row" : "md:flex-row-reverse"} justify-between `}
                    >
                        <div className="md:w-1/2">
                            <h2 className="text-3xl font-semibold text-[var(--maintext1)]">{project.name}</h2>

                            <div className="flex flex-row mb-2">
                                {project.skills.map((projSkill, idx) => {
                                    const skillObj = skills.find((s) => s.slug === projSkill.skill_slug);
                                    if (!skillObj) return null;

                                    return (
                                        <p key={idx}
                                           className="text-md text-[var(--maintext3)] space-y-1">{idx == 0 ? skillObj.skill : ', ' + skillObj.skill}</p>
                                    );
                                })}
                            </div>

                            <div className="space-y-2 text-[var(--maintext2)]">
                                {project.summary.map((paragraph, idx) => (
                                    <p key={idx} dangerouslySetInnerHTML={{__html: paragraph}}/>
                                ))}
                            </div>
                        </div>

                        <div
                            className={`flex flex-col items-center ${idx % 2 === 0 ? "md:items-end" : "md:items-start"} md:w-1/2 justify-center`}>
                            {project.image && (
                                <Image
                                    src={project.image}
                                    alt={project.name}
                                    width={300}
                                    height={300}
                                    className="object-contain border rounded-3xl "
                                />
                            ) || project.video && (
                                <iframe src={project.video} allowFullScreen className="rounded-3xl h-30 w-53 md:h-40 md:w-71 lg:h-50 lg:w-89"/>
                            )}
                        </div>

                    </Link>
                ))}
            </div>
        </>
    );
}
