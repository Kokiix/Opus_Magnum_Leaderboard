import { createClient } from '@supabase/supabase-js';

export const POST = async ({ request }) => {
    // TODO: move to env file
    if (request.headers.get("very_secret_key") != "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=")
        return new Response(null, { status: 404 });
    const newScores = await request.json();
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');

    const levels = newScores.stats.map(s => s.level);
    const steamId = newScores.stats[0]?.steam_id;

    const { data: existingScoreList } = await supabase
        .from('scores')
        .select('level, sum')
        .eq('steam_id', steamId)
        .in('level', levels);
    const existingScoreMap = Object.fromEntries(existingScoreList.map(s => [s.level, s.sum]));

    const scoresToUpsert = newScores.stats.filter(s => {
        const prevSum = existingScoreMap[s.level];
        return !prevSum || s.sum < prevSum;
    })
    if (scoresToUpsert.length > 0) {
        await supabase
            .from('scores')
            .upsert(scoresToUpsert);
        return new Response(null, { status: 201 });
    }
    return new Response(null, { status: 200 });
}
