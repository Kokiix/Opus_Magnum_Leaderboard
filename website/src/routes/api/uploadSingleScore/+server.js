import { createClient } from '@supabase/supabase-js';

export const POST = async ({ request }) => {
    // TODO: move to env file
    if (request.headers.get("very_secret_key") != "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=")
        return new Response(null, { status: 404 });
    const new_score = await request.json();
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');

    // Check for existence
    const { data: existingSum, error } = await supabase
        .from('scores')
        .select('sum')
        .eq('level', new_score.level)
        .eq('steam_id', new_score.steam_id)
        .maybeSingle();

    if (error)
        return new Response(`Failed to get existing score: ${error.message}`, { status: 500 });

    // Insert
    console.log(existingSum);
    if (!existingSum || new_score.sum < existingSum) {
        const { error } = await supabase
            .from('scores')
            .insert(new_score);
        if (error)
            return new Response(`Failed to insert new score: ${error.message}`, { status: 500 });
        return new Response(null, { status: 201 });
    }

    return new Response(null, { status: 200 });
}
