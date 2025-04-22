'use client';

import globals from "@/data/globals.json";
import {useEffect, useRef, useState} from "react";

export default function Footer() {
    const [visible, setVisible] = useState(true)
    const timeoutRef = useRef<NodeJS.Timeout | null>(null)
    const [hasMounted, setHasMounted] = useState(false)

    useEffect(() => {
        setHasMounted(true)
    }, [])

    useEffect(() => {
        if (!hasMounted) return

        const handleScroll = () => {
            const isMobile = window.innerWidth < 768
            if (!isMobile) return

            setVisible(true)

            if (timeoutRef.current) clearTimeout(timeoutRef.current)

            timeoutRef.current = setTimeout(() => {
                setVisible(false)
            }, 2500)
        }

        window.addEventListener('scroll', handleScroll)
        return () => {
            window.removeEventListener('scroll', handleScroll)
            if (timeoutRef.current) clearTimeout(timeoutRef.current)
        }
    }, [hasMounted])

    if (!hasMounted) return null // prevents hydration error

    return (
        <footer className={`fixed bottom-0 bg-gray-800 text-white pt-4 w-full transition-transform duration-500 ease-in-out ${visible ? 'translate-y-0' : 'translate-y-full'}`}>
            <div className="container mx-auto text-center">
                <div className="flex justify-center items-center space-x-6">
                    {globals.footer.map((item, index) => {
                        if (item.link) {
                            return (
                                <a
                                    key={index}
                                    href={item.link}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="text-white hover:text-blue-600 transition-colors"
                                >
                                    <div
                                        className="size-8 [&>svg]:w-full [&>svg]:h-full [&>svg]:object-contain"
                                        dangerouslySetInnerHTML={{__html: item.icon}}
                                    />
                                </a>
                            )
                        } else if (item.download) {
                            return (
                                <a
                                    key={index}
                                    href={`/files/${item.download}`}
                                    className="text-white hover:text-blue-600 transition-colors"
                                    download={item.download}
                                >
                                    {item.Text}
                                </a>
                            )
                        }
                        return null
                    })}
                </div>
                <p className="text-sm text-gray-400">© 2025 Xavier Garay. All rights reserved.</p>
            </div>
        </footer>
    )
}