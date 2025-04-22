import education from "@/data/content/Education.json";
import Image from "next/image";


export default function Education() {
    return (
        <>
            <h1 className="custom_backdrop text-center mb-10 ">
                <p className="text-3xl">Select a degree to download transcripts</p>
                <p className="text-[var(--maintext3)]">Transcripts from the same school will be on the same document</p>
            </h1>
            <div className="flex flex-col gap-8 items-center">
                {education.map((degree) => (
                    <a
                        key={degree.slug}
                        href={"files/" + degree.transcript}
                        download={degree.transcript}
                        className="custom_backdrop custom_backdrop_link w-full cursor-pointer"
                    >
                        <div
                            className="size-full">
                            <div className="flex flex-row justify-between items-center">
                                <div className="w-[45%] md:w-[35%] lg:w-[25%]">
                                    <h3 className={`text-xl font-semibold ${degree.comments ? "" : "mb-2"}`}>{degree.major}</h3>

                                    {degree.comments && (
                                        <p className="text-sm text-[var(--maintext2)] mb-3">{degree.comments}</p>
                                    )}

                                    <p className="text-sm text-[var(--maintext2)] mb-1">{degree.school}</p>

                                    <p className="text-sm text-[var(--maintext3)] mb-2">{degree.dates}</p>
                                </div>

                                <p className="w-[40%] hidden lg:block">{degree.summary}</p>

                                <div className="w-[45%] md:w-[35%] lg:w-[25%] flex flex-col items-center">
                                    <div className="flex flex-col justify-center h-[100px] w-[100px] ">
                                        <Image
                                            src={degree.logo}
                                            width={500}
                                            height={500}
                                            alt={degree.school}
                                        />
                                    </div>
                                </div>

                            </div>
                        </div>
                    </a>
                ))}
            </div>
        </>

    );
}
