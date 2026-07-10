# Branding System

## Brand Intent

SUPER IDE should feel fast, sharp, practical, and modern. Orange is the product
signature, but the workbench remains dark, quiet, and code-first. Orange marks
identity, focus, primary actions, cursor, selection, and AI accents.

## Source Assets

- App mark: `extra/images/superide/superide-mark.png`
- Glow lockup: `extra/images/superide/superide-glow-lockup.png`
- Default theme: `defaults/super-orange-dark.toml`

Use the app mark for application icon generation, splash surfaces, installer
graphics, and about surfaces. Use the glow lockup for marketing, release notes,
or launch imagery. The editor workbench should not use large glowing art as a
panel background.

## Color Tokens

```css
--super-orange-50:  #FFF7ED;
--super-orange-100: #FFEDD5;
--super-orange-200: #FED7AA;
--super-orange-300: #FDBA74;
--super-orange-400: #FB923C;
--super-orange-500: #F97316;
--super-orange-600: #EA580C;
--super-orange-700: #C2410C;
--super-orange-800: #9A3412;
--super-orange-900: #7C2D12;

--bg-primary:   #0A0A0A;
--bg-secondary: #111111;
--bg-tertiary:  #18181B;
--bg-panel:     #1F1F23;

--border:       #2A2A2E;
--border-soft:  #202024;

--text-primary:   #FAFAFA;
--text-secondary: #A1A1AA;
--text-muted:     #71717A;

--success: #22C55E;
--warning: #F59E0B;
--error:   #EF4444;
--info:    #3B82F6;
```

## Agent Colors

```css
--agent-planner:  #F97316;
--agent-coder:    #22C55E;
--agent-reviewer: #3B82F6;
--agent-debug:    #EF4444;
--agent-research: #A855F7;
```

## Workbench Rules

- Backgrounds stay near black and zinc.
- Primary actions use `#F97316`.
- Selections use `rgba(249,115,22,0.25)`.
- The cursor uses `#F97316`.
- Chat user bubbles use orange; assistant bubbles use neutral dark panels.
- Panels use flat surfaces and 6px radius or less.
- Avoid full-panel orange gradients in the IDE.
- Keep AI visual treatment subtle until the AI surface is opened.

## Implementation Status

- `SUPER Orange Dark` theme has been added and set as default.
- Provided PNG assets have been copied into the foundation tree.
- Visible app strings have started moving from Lapce to SUPER IDE.

Next branding tasks:

- Generate platform icons from `superide-mark.png`.
- Replace the about popup logo path with a SUPER-compatible SVG or PNG view.
- Update Linux desktop metadata, macOS bundle metadata, Windows icon resources,
  and package names.
- Add a small activity-bar AI icon that activates the lazy AI surface.
