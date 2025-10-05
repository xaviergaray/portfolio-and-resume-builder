import experiences from "@/data/content/Work-Experience.json";
import skills from "@/data/content/Skills.json";
import Link from "next/link";
import Image from "next/image";
import React from "react";
import {notFound} from "next/navigation";

export default async function Page({ params, }: { params: Promise<{ slug: string }> }) {
    const { slug } = await params;
    const experience = experiences.find((e) => e.slug === slug);
    const header_style = "text-xl font-semibold mb-2 text-[var(--maintext1)]";

    if (!experience) return notFound();

    return (
        <div className="flex flex-col gap-10">
            <div className="bg-[var(--backdrop)] p-6 rounded-lg shadow">
                <div className="flex flex-row justify-between items-center">
                    <div>
                        <h1 className={header_style}>{experience.title}</h1>

                        <p className="text-sm mb-1 text-[var(--maintext2)]">{experience.company}</p>

                        <p className="text-sm text-[var(--maintext3)] mb-2">{experience.date_range}</p>
                    </div>
                    <Image
                        src={experience.logo}
                        alt={experience.company}
                        width={100}
                        height={100}
                        className="max-w-[50vw] max-h-[60px] object-contain"
                    />
                </div>

                <ul className="list-disc list-inside text-sm text-[var(--maintext4)] space-y-1">
                    {experience.details.map((detail) => (
                        <li key={detail} dangerouslySetInnerHTML={{__html: detail}}></li>
                    ))}
                </ul>
            </div>



            <div className="bg-[var(--backdrop)] p-6 rounded-lg shadow">
                <h1 className={header_style}>Relevant Skills</h1>
                <ul className="space-y-10">
                    {experience.skills.map((expSkill, idx) => (
                        <li key={idx} className="flex flex-col gap-2">
                            <div className="flex flex-wrap gap-2">
                                {expSkill.skill_slugs.map((skill_slug) => {
                                    const skillObj = skills.find((s) => s.slug === skill_slug);
                                    if (!skillObj) return null;

                                    return (
                                        <Link
                                            key={skill_slug}
                                            href={`/skills/${skill_slug}`}
                                            className="custom_button w-[auto] h-[3rem] px-10"
                                        >
                                            {skillObj.skill}
                                        </Link>
                                    );
                                })}
                            </div>
                            <p className="text-md text-[var(--maintext2)] space-y-1">{expSkill.description}</p>
                        </li>
                    ))}
                </ul>
            </div>
        </div>
)
    ;
}

export function generateStaticParams() {
    return experiences.map((e) => ({
        slug: e.slug,
    }));
}