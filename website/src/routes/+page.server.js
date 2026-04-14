import { createClient } from '@supabase/supabase-js';
import rawLevelNames from '$lib/assets/level_names.txt?raw';
import rawChapterNames from '$lib/assets/chapter_names.txt?raw'
import { SUPABASE_KEY, SUPABASE_URL } from '$env/static/private';

export async function load() {
    const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);

    const steamID_to_username = { "76561198818284135": "koki", "76561198861611607": "pooki", "76561198141313239": "noki" };
    const { data: scores } = await supabase
        .from('scores')
        .select();

    let lvName_to_scoreObj = {};
    scores.forEach(score => {
        score.username = steamID_to_username[score.steam_id];
        if (lvName_to_scoreObj[score.level]) {
            if (score.sum < lvName_to_scoreObj[score.level].sum)
                lvName_to_scoreObj[score.level] = score;
        } else {
            lvName_to_scoreObj[score.level] = score;
        }
    });

    let levels = rawLevelNames
        .split('---')
        .map(x => x.trim().split('\n'))
        .map(lvlsInChapter =>
            lvlsInChapter.map(prettyLvName => {
                const slugLvName = prettyLvName.split(' ').map(w => w.toLowerCase()).join('-')
                if (lvName_to_scoreObj[slugLvName]) {
                    lvName_to_scoreObj[slugLvName].level = prettyLvName;
                    return lvName_to_scoreObj[slugLvName]
                };
                return { "level": prettyLvName };
            })
        );

    const chapterList = rawChapterNames.split('\n').map((chapName, index) => {
        return {
            "title": chapName,
            "levels": levels[index]
        };
    });

    return { chapterList };
}