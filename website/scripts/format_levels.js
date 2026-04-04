import fs from 'node:fs';

// AI generated

const levelsPath = 'src/lib/assets/level_names.txt';
const sectionsPath = 'src/lib/assets/sections.txt';

try {
    const content = fs.readFileSync(levelsPath, 'utf-8');
    const lines = content.split(/\r?\n/).map(l => l.trim()).filter(Boolean);

    const sections = [];
    const formattedLines = [];

    const slugify = (name) => {
        return name
            .toLowerCase()
            .replace(/['']/g, '')
            .replace(/[^a-z0-9]+/g, '-')
            .replace(/-+/g, '-')
            .replace(/^-+|-+$/g, '');
    };

    const isSection = (line) => {
        return line.startsWith('Chapter') ||
            line.startsWith('Journal') ||
            line === 'Appendix';
    };

    lines.forEach(line => {
        if (isSection(line)) {
            sections.push(line);
            formattedLines.push('---'); // The divider marker
        } else {
            formattedLines.push(slugify(line));
        }
    });

    // Write the level names with markers
    fs.writeFileSync(levelsPath, formattedLines.join('\n'), 'utf-8');
    // Write the actual section names to their own file
    fs.writeFileSync(sectionsPath, sections.join('\n'), 'utf-8');

    console.log(`Successfully updated ${levelsPath} with markers.`);
    console.log(`Created ${sectionsPath} with ${sections.length} section headers.`);
} catch (error) {
    console.error('Error processing levels:', error.message);
    process.exit(1);
}
