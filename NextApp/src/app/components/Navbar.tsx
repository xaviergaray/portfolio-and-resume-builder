'use client'
import NavButton from "@/app/components/NavButton";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBars, faTimes } from '@fortawesome/free-solid-svg-icons'
import {useEffect, useState} from "react";
import { usePathname } from 'next/navigation'; // Use the new usePathname hook from next/navigation

export default function Navbar() {
    const [isOpen, setIsOpen] = useState(false);
    const pathname = usePathname(); // Get the current path

    const orderedSections = [
        "My-Portfolio",
        "Work-Experience",
        "Skills",
        "Education",
        "Certifications",
        "Projects",
    ];

    useEffect(() => {
        setIsOpen(false);
    }, [pathname]);

    return (
        <nav className="fixed top-0 right-0 left-0">
            <header className="bg-white shadow">
                <div className="container mx-auto py-4 px-4 flex justify-between items-center">
                    {/* Logo / Home */}
                    <NavButton
                        text={orderedSections[0].replace('-', ' ')}
                        href="/"
                        style_selected="hover:text-blue-600"
                        style_unselected="hover:text-blue-600"
                        style_general="text-black text-2xl font-bold focus:outline-none transition-colors duration-200"
                    />

                    {/* Hamburger icon (mobile only) */}
                    <div className="md:hidden">
                        <button onClick={() => setIsOpen(!isOpen)} className="text-gray-600 focus:outline-none">
                            <FontAwesomeIcon icon={isOpen ? faTimes : faBars} size="lg" />
                        </button>
                    </div>

                    {/* Full nav links (desktop) */}
                    <div className="hidden md:flex gap-4">
                        {orderedSections.slice(1).map((text, i) => (
                            <NavButton
                                key={i + 1}
                                text={text.replace('-', ' ')}
                                href={`/${text.toLowerCase()}`}
                                style_selected="text-blue-600 border-b-2 border-blue-600"
                                style_unselected="text-gray-600 hover:text-blue-600"
                                style_general="pb-1 focus:outline-none transition-colors duration-200"
                            />
                        ))}
                    </div>
                </div>

                {/* Mobile nav links (dropdown menu) */}
                <div
                    className={`px-4 relative md:hidden overflow-hidden transition-[max-height] duration-500 ease-in-out ${
                        isOpen ? 'max-h-96' : 'max-h-0'
                    }`}
                >
                    {orderedSections.slice(1).map((text, i) => (
                        <NavButton
                            key={i + 1}
                            text={text.replace('-', ' ')}
                            href={`/${text.toLowerCase()}`}
                            style_selected="text-blue-600"
                            style_unselected="text-gray-600 hover:text-blue-600"
                            style_general="text-lg focus:outline-none transition-colors duration-200"
                        />
                    ))}
                </div>

            </header>
        </nav>
    )
}