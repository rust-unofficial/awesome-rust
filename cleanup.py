# -*- coding: utf-8 -*-
"""
Cleans up `README.md`.
"""

import codecs


def fix_dashes(lines):
    """
    """

    fixed_lines = []

    # Distinguish between the prologue and the content.
    within_content = False

    # Iterate over the awesome lines.
    for line in lines:

        # The current line is within the content.
        if within_content:

            # Adjust the dash.
            fixed_lines.append(line.replace(u' - ', u' — '))
        #
        # The current line is within the prologue.
        else:
            # The prologue has ended.
            if line.startswith(u'## Applications'):
                within_content = True

            # Leave the current line unmodified.
            fixed_lines.append(line)

    return fixed_lines

# end def fix_dashes


# Read the awesome file.
with codecs.open('README.md', encoding='utf8') as awesome_file:
    awesome_lines = awesome_file.readlines()

# Fix the dashes.
awesome_lines = fix_dashes(awesome_lines)

# Write the awesome file.
with codecs.open('README.md', 'wb', encoding='utf8') as awesome_file:
    awesome_file.writelines(awesome_lines)
