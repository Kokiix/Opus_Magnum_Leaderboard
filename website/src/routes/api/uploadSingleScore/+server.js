import { createClient } from '@supabase/supabase-js';

export const POST = async ({ request }) => {
    // TODO: move to env file
    if (request.headers.get("very_secret_key") != "QCR8VE5UNSo6XHVOa11rX0A1eXxJQW5ubkBRLWEjLS9tSHNuLjx0XC4nLEYrLTo=")
        return new Response(null, { status: 404 });
    const data = await request.json();
    const supabase = createClient( // TODO: change url when going public, move to env file
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');
    // Check for existence
    // Insert
    const { error } = await supabase
        .from('scores')
        .insert(data);

    if (error)
        return new Response(error.message);
    else
        return new Response(null, { status: 200 });
}
