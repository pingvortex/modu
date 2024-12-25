import { Home, Baseline, File, FileBox, Equal, TriangleAlert, Server } from "lucide-svelte"

export default {
    pages: [
        {
            "path": "quickstart",
            "title": "Quickstart",
            "icon": Home,
        }, 
        {
            "path": "basics",
            "title": "Basics",
            "icon": Baseline,
        },
        {
            "path": "imports",
            "title": "Imports",
            "icon": FileBox,
        },
        {
            "path": "math",
            "title": "Math",
            "icon": Equal,
        },
        {
            "path": "file",
            "title": "File I/O",
            "icon": File,
        },
        {
            "path": "limitations",
            "title": "Limitations",
            "icon": TriangleAlert,
        },
        {
            "path": "server",
            "title": "Server",
            "icon": Server,
        }
    ]
}