import fs from 'node:fs';

// AI Generated

const filePath = 'src/lib/assets/level_names.txt';

try {
    const content = fs.readFileSync(filePath, 'utf-8');

    const formatLevelName = (name) => {
        return name
            .toLowerCase()
            .replace(/['']/g, '')         // Remove apostrophes
            .replace(/[^a-z0-9]+/g, '-')  // Replace non-alphanumeric with hyphens
            .replace(/-+/g, '-')          // Collapse consecutive hyphens
            .replace(/^-+|-+$/g, '');     // Trim hyphens from ends
    };

    const lines = content.split(/\r?\n/).filter(line => line.trim() !== '');
    const formattedLines = lines.map(formatLevelName);

    fs.writeFileSync(filePath, formattedLines.join('\n'), 'utf-8');
    console.log(`Successfully formatted ${formattedLines.length} level names in ${filePath}`);
} catch (error) {
    console.error('Error processing file:', error.message);
    process.exit(1);
}
