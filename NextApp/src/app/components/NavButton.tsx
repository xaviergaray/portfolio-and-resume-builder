'use client'

import Link from 'next/link'
import { usePathname} from "next/navigation";


export default function NavButton({text, href, style_selected, style_unselected, style_general}: { text: string; href: string; style_selected: string; style_unselected: string; style_general: string;}) {
    const pathname = usePathname()
    const isActive = href === '/' ? pathname === '/' : pathname.startsWith(href)

    return (
        <Link href={href} className={style_general + " " + (isActive ? style_selected : style_unselected)}>
            <p>{text}</p>
        </Link>
    )
}