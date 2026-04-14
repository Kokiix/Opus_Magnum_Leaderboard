import { SUPABASE_KEY, SUPABASE_URL } from '$env/static/private';
import { createClient } from '@supabase/supabase-js';

export const POST = async ({ request }) => {
    if (request.headers.get("very_secret_key") != "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=")
        return new Response(null, { status: 404 });
    const newScore = await request.json();
    const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);

    // Check for existence
    const { data: existingScore, error } = await supabase
        .from('scores')
        .select('sum')
        .eq('level', newScore.level)
        .eq('steam_id', newScore.steam_id)
        .maybeSingle();

    if (error)
        return new Response(`Failed to get existing score: ${error.message}`, { status: 500 });

    // Insert
    if (!existingScore || newScore.sum < existingScore.sum) {
        const { error } = await supabase
            .from('scores')
            .upsert(newScore);
        if (error)
            return new Response(`Failed to insert new score: ${error.message}`, { status: 500 });
        return new Response(null, { status: 201 });
    }

    return new Response(null, { status: 200 });
}
