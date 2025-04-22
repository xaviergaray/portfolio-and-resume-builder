import skills from "@/data/content/Skills.json";
import Link from "next/link";
import {SVGProps} from "react";


function Stars({ level }: { level: number }) {
    const starStyle = 'text-2xl text-red-500';
    const fullStars = Math.floor(level / 2);
    const halfStar = level % 2;
    const emptyStars = 5 - fullStars - halfStar;
    const starSize = '1rem';

    const FullStarImage = (props: SVGProps<SVGSVGElement>) => (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={starSize}
            height={starSize}
            fill="currentColor"
            viewBox="0 0 32 32"
            {...props}
        >
            <title>{"star-filled"}</title>
            <path d="M30.859 12.545a1.253 1.253 0 0 0-1.189-.864h-9.535l-2.946-9.067a1.3 1.3 0 0 0-2.373-.008l-.003.008-2.946 9.067H2.333a1.25 1.25 0 0 0-.738 2.259l.004.002 7.713 5.603-2.946 9.068a1.25 1.25 0 0 0 1.927 1.396l-.004.002 7.714-5.605 7.713 5.605a1.25 1.25 0 0 0 1.921-1.408l.003.009-2.947-9.066 7.715-5.604a1.253 1.253 0 0 0 .452-1.406l.003.009z" />
        </svg>
    )

    const HalfStarImage = (props: SVGProps<SVGSVGElement>) => (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={starSize}
            height={starSize}
            fill="currentColor"
            viewBox="0 0 32 32"
            {...props}
        >
            <title>{"star-half-stroke-filled"}</title>
            <path d="M30.859 12.545a1.253 1.253 0 0 0-1.189-.864h-9.535l-2.946-9.067a1.252 1.252 0 0 0-1.584-.799l.009-.003c-.376.13-.664.427-.779.8l-.002.009-.021-.007-2.946 9.067H2.332a1.25 1.25 0 0 0-.738 2.259l.004.002 7.713 5.603-2.946 9.068a1.25 1.25 0 0 0 1.927 1.396l-.004.002 7.714-5.605 7.713 5.605a1.25 1.25 0 0 0 1.921-1.408l.003.009-2.947-9.066 7.715-5.604a1.253 1.253 0 0 0 .452-1.406l.003.009zm-10.373 5.512a1.255 1.255 0 0 0-.452 1.408l-.003-.009 2.039 6.271-5.336-3.877a1.215 1.215 0 0 0-.694-.215l-.042.001H16V7.047l2.037 6.272c.169.505.637.863 1.189.863h6.596z" />
        </svg>
    )

    const EmptyStarImage = (props: SVGProps<SVGSVGElement>) => (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={starSize}
            height={starSize}
            fill="currentColor"
            viewBox="0 0 32 32"
            {...props}
        >
            <title>{"star"}</title>
            <path d="M30.859 12.545a1.253 1.253 0 0 0-1.189-.864h-9.535l-2.946-9.067a1.3 1.3 0 0 0-2.373-.008l-.003.008-2.946 9.067H2.333a1.25 1.25 0 0 0-.738 2.259l.004.002 7.713 5.603-2.946 9.068a1.25 1.25 0 0 0 1.927 1.396l-.004.002 7.714-5.605 7.713 5.605a1.25 1.25 0 0 0 1.921-1.408l.003.009-2.947-9.066 7.715-5.604a1.253 1.253 0 0 0 .452-1.406l.003.009zm-10.373 5.512a1.255 1.255 0 0 0-.452 1.408l-.003-.009 2.039 6.271-5.336-3.877c-.203-.149-.458-.238-.734-.238s-.531.089-.738.241l.004-.002-5.336 3.877 2.038-6.271a1.25 1.25 0 0 0-.45-1.396l-.004-.002-5.335-3.876h6.596c.552 0 1.02-.358 1.185-.854l.003-.009 2.038-6.272 2.037 6.272c.169.505.637.863 1.189.863h6.596z" />
        </svg>
    )

    return (
        <div className="flex items-center mb-2">
            {/* Full stars */}
            {Array.from({length: fullStars}).map((_, i) => (
                <span key={`full-${i}`} className={starStyle}><FullStarImage /></span>
            ))}

            {/* Half star */}
            {halfStar === 1 && (
                <span className={starStyle}><HalfStarImage /></span>
            )}

            {/* Empty stars */}
            {Array.from({length: emptyStars}).map((_, i) => (
                <span key={`empty-${i}`} className={starStyle}><EmptyStarImage /></span>
            ))}
        </div>
    )
}

export default function Skills() {
    return (
        <>
            <h1 className="custom_backdrop text-3xl text-center mb-10">Select a skill to see what experience I have with it</h1>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 w-full">
                {[...skills].sort((a, b) => a.skill.localeCompare(b.skill)).map((skill) => (
                    <Link key={skill.slug} href={`/skills/${skill.slug}`}
                          className="custom_backdrop custom_backdrop_link flex justify-between items-center">
                        <div>
                            <h3 className="text-xl font-semibold mb-1">{skill.skill}</h3>

                            <Stars level={skill.level}/>

                            <p className="text-sm text-[var(--maintext2)] mb-2 w-[80%] hidden md:block">{skill.summary}</p>
                        </div>

                        <div>
                            <div
                                className="w-12 h-12 [&>svg]:w-full [&>svg]:h-full [&>svg]:object-contain"
                                dangerouslySetInnerHTML={{__html: skill.icon}}
                            />
                        </div>
                    </Link>
                ))}
            </div>
        </>
    );
}
