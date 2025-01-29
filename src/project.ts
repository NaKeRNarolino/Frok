import type {AutosortTag} from "./autosort_tags";

export interface ProjectConfig {
    project_root: string;
    autosort_tags: AutosortTag[]
}