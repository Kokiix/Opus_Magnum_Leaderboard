import { createClient } from '@supabase/supabase-js';

export const POST = async ({ request }) => {
    const data = await request.json();
    const supabase = createClient(
        'https://zeddvrudhdrakfbmzinh.supabase.co',
        'sb_publishable_coM9-yUpcpkfpQfBf7y6Ug_HdbicKL3');
    // Check for existence
    // Insert
    const { error } = await supabase
        .from('scores')
        .insert(data);

    return new Response(error.message);
}
