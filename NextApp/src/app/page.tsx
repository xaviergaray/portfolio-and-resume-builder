import introContent from "@/data/content/My-Portfolio.json";

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-center w-full gap-[3rem]">
      {introContent.map((section, index) => (
          <div key={index} className="custom_backdrop p-6 flex flex-col gap-[2rem]">
            <h1 className="text-3xl font-semibold"> {section.header} </h1>
            <div className="space-y-4">
              {section.information.map((paragraph, idx) => (
                  <p key={idx} className="text-[var(--maintext2)]" dangerouslySetInnerHTML={{__html: paragraph}}></p>
              ))}
            </div>
          </div>
      ))}
    </main>
  );
}
