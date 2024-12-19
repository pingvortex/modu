import { Home, Baseline, File, Equal, TriangleAlert, Server } from "lucide-svelte"

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
            "icon": File,
        },
        {
            "path": "math",
            "title": "Math",
            "icon": Equal,
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