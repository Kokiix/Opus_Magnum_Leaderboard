import { createClient } from '@supabase/supabase-js';
import rawLevelNames from '$lib/assets/level_names.txt?raw'

export async function load() {
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');
    const { data: scores } = await supabase
        .from('scores')
        .select();

    const levelNames = rawLevelNames.split('\n');

    let levels = Object.fromEntries(levelNames.map(name => [name, { "level": name }]));
    const steamID_to_username = { "76561198818284135": "koki" };
    scores.forEach(s => {
        s.username = steamID_to_username[s.steam_id];
        levels
    });
    return { scores };
}