import { createClient } from '@supabase/supabase-js';
import rawLevelNames from '$lib/assets/level_names.txt?raw';
import rawChapterNames from '$lib/assets/chapter_names.txt?raw'

export async function load() {
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');

    const steamID_to_username = { "76561198818284135": "koki", "76561198861611607": "pooki", "76561198141313239": "noki" };
    const { data: scores } = await supabase
        .from('scores')
        .select();

    const lvName_to_scoreObj = Object.fromEntries(scores.map(score => {
        score.username = steamID_to_username[score.steam_id];
        return [score.level, score];
    }));

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