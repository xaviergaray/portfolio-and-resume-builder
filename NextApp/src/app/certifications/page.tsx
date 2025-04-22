import certifications from "@/data/content/Certifications.json";

export default function Certifications() {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {certifications.map((certification) => (
                <div key={certification.title} >
                    <div className="custom_backdrop h-full flex flex-col justify-between">
                        <div>
                            <h1 className="font-bold text-xl">{certification.title}</h1>
                            <br/>
                            <div className="w-full h-[3px] bg-[var(--maintext2)]"></div>
                            <br/>
                            <p className="text-[var(--maintext2)]">{certification.description}</p>
                        </div>

                        <br/>
                        <a href={certification.link} target="_blank" rel="noopener noreferrer"
                           className="w-full flex flex-col items-center ">
                            <div
                                className="custom_button w-[70%] h-[4rem]">
                                <p className="text-[var(--badge-button-text)]">Credly Badge</p>
                            </div>
                        </a>
                    </div>
                </div>
            ))}
        </div>
    );
}
