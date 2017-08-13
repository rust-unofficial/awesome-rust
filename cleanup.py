# -*- coding: utf-8 -*-
"""
Cleans up `README.md`.
"""

import codecs


def fix_dashes(lines):
    """
    """

    fixed_lines = []

    within_content = False

    for line in lines:

        # Only touch the content, not the prologue.
        if within_content:
            fixed_lines.append(line.replace(u' - ', u' — '))
        #
        else:
            if line.startswith(u'## Applications'):
                within_content = True

            fixed_lines.append(line)

    return fixed_lines

# end def fix_dashes


# Read the awesome file.
with codecs.open('README.md', encoding='utf8') as awesome_file:
    awesome_lines = awesome_file.readlines()

awesome_lines = fix_dashes(awesome_lines)

# Write the awesome file.
with codecs.open('README.md', 'wb', encoding='utf8') as awesome_file:
    awesome_file.writelines(awesome_lines)
