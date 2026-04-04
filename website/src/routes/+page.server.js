import { createClient } from '@supabase/supabase-js';
import rawLevelNames from '$lib/assets/level_names.txt?raw';
import rawChapterNames from '$lib/assets/chapter_names.txt?raw'

export async function load() {
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');

    const steamID_to_username = { "76561198818284135": "koki" };
    const { data: scores } = await supabase
        .from('scores')
        .select();

    console.log(scores);
    const lvName_to_scoreObj = Object.fromEntries(scores.map(score => {
        score.username = steamID_to_username[score.steam_id];
        return [score.level, score];
    }));

    let levels = rawLevelNames
        .split('---')
        .map(x => x.split('\n'))
        .map(lvlsInChapter =>
            lvlsInChapter.map(lvlName => {
                if (lvName_to_scoreObj[lvlName]) return lvName_to_scoreObj[lvlName];
                return { "level": lvlName };
            })
        );

    console.log(levels);

    const chapterList = rawChapterNames.split('\n').map((chapName, index) => {
        return {
            "title": chapName,
            "levels": levels[index]
        };
    });

    return { chapterList };
}