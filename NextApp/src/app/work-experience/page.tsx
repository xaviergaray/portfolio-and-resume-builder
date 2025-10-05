import experience from "@/data/content/Work-Experience.json";
import Image from "next/image";
import Link from "next/link";

export default function WorkExperience() {
    return (
        <>
            <h1 className="custom_backdrop text-3xl text-center mb-10">Select an experience to learn more</h1>
            <div className="flex flex-col gap-10">
                {experience.map((experience) => (
                    <Link key={experience.slug} href={`/work-experience/${experience.slug}`} className="custom_backdrop custom_backdrop_link">
                            <div className="flex flex-row justify-between items-center">
                                <div>
                                    <h3 className="text-xl font-semibold mb-2 text-[var(--maintext1)]">{experience.title}</h3>

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
                    </Link>
                ))}
            </div>
        </>
    );
}
