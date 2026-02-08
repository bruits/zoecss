# Tailwind CSS v4 — Rule Groups

This document tracks ZoeCSS implementation progress of all Tailwind CSS v4 utility groups. Each group is marked as not started `[ ]`, in progress `[~]`, or done `[x]`.

## Layout

- [~] Aspect Ratio (`aspect-auto`, `aspect-square`, `aspect-video`, `aspect-[ratio]`) — keyword + theme values done, arbitrary values missing
- [ ] Container (`container`)
- [x] Columns (`columns-1…12`, `columns-auto`, `columns-3xs…7xl`)
- [x] Break After (`break-after-auto`, `break-after-avoid`, `break-after-all`, `break-after-page`, `break-after-column`)
- [x] Break Before (`break-before-auto`, `break-before-avoid`, `break-before-all`, `break-before-page`, `break-before-column`)
- [x] Break Inside (`break-inside-auto`, `break-inside-avoid`, `break-inside-avoid-page`, `break-inside-avoid-column`)
- [x] Box Decoration Break (`box-decoration-clone`, `box-decoration-slice`)
- [x] Box Sizing (`box-border`, `box-content`)
- [x] Display (`block`, `inline`, `inline-block`, `flex`, `inline-flex`, `grid`, `inline-grid`, `table`, `table-*`, `flow-root`, `contents`, `list-item`, `hidden`)
- [x] Float (`float-left`, `float-right`, `float-none`, `float-start`, `float-end`)
- [x] Clear (`clear-left`, `clear-right`, `clear-both`, `clear-none`, `clear-start`, `clear-end`)
- [x] Isolation (`isolate`, `isolation-auto`)
- [x] Object Fit (`object-contain`, `object-cover`, `object-fill`, `object-none`, `object-scale-down`)
- [x] Object Position (`object-bottom`, `object-center`, `object-left`, `object-top`, …)
- [x] Overflow (`overflow-auto`, `overflow-hidden`, `overflow-clip`, `overflow-visible`, `overflow-scroll`, `overflow-x-*`, `overflow-y-*`)
- [x] Overscroll Behavior (`overscroll-auto`, `overscroll-contain`, `overscroll-none`, `overscroll-x-*`, `overscroll-y-*`)
- [x] Position (`static`, `fixed`, `absolute`, `relative`, `sticky`)
- [~] Top / Right / Bottom / Left / Inset (`top-*`, `right-*`, `bottom-*`, `left-*`, `inset-*`, `inset-x-*`, `inset-y-*`, `start-*`, `end-*`) — spacing theme values, `auto`/fraction/negative values missing
- [x] Visibility (`visible`, `invisible`, `collapse`)
- [x] Z-Index (`z-0`, `z-10…50`, `z-auto`, `z-[value]`)

## Flexbox & Grid

- [~] Flex Basis (`basis-0…96`, `basis-auto`, `basis-full`, `basis-1/2…11/12`) — spacing theme + keyword values done, fraction values missing
- [x] Flex Direction (`flex-row`, `flex-row-reverse`, `flex-col`, `flex-col-reverse`)
- [x] Flex Wrap (`flex-wrap`, `flex-wrap-reverse`, `flex-nowrap`)
- [x] Flex (`flex-1`, `flex-auto`, `flex-initial`, `flex-none`)
- [x] Flex Grow (`grow`, `grow-0`)
- [x] Flex Shrink (`shrink`, `shrink-0`)
- [x] Order (`order-1…12`, `order-first`, `order-last`, `order-none`)
- [x] Grid Template Columns (`grid-cols-1…12`, `grid-cols-none`, `grid-cols-subgrid`, `grid-cols-[value]`)
- [x] Grid Column Span / Start / End (`col-span-1…12`, `col-span-full`, `col-start-*`, `col-end-*`, `col-auto`, `col-[value]`, `col-start-[value]`, `col-end-[value]`)
- [x] Grid Template Rows (`grid-rows-1…12`, `grid-rows-none`, `grid-rows-subgrid`, `grid-rows-[value]`)
- [x] Grid Row Span / Start / End (`row-span-1…12`, `row-span-full`, `row-start-*`, `row-end-*`, `row-auto`, `row-[value]`, `row-start-[value]`, `row-end-[value]`)
- [x] Grid Auto Flow (`grid-flow-row`, `grid-flow-col`, `grid-flow-dense`, `grid-flow-row-dense`, `grid-flow-col-dense`)
- [x] Grid Auto Columns (`auto-cols-auto`, `auto-cols-min`, `auto-cols-max`, `auto-cols-fr`)
- [x] Grid Auto Rows (`auto-rows-auto`, `auto-rows-min`, `auto-rows-max`, `auto-rows-fr`)
- [x] Gap (`gap-*`, `gap-x-*`, `gap-y-*`)
- [x] Justify Content (`justify-normal`, `justify-start`, `justify-end`, `justify-center`, `justify-between`, `justify-around`, `justify-evenly`, `justify-stretch`)
- [x] Justify Items (`justify-items-start`, `justify-items-end`, `justify-items-center`, `justify-items-stretch`, `justify-items-normal`)
- [x] Justify Self (`justify-self-auto`, `justify-self-start`, `justify-self-end`, `justify-self-center`, `justify-self-stretch`)
- [x] Align Content (`content-normal`, `content-center`, `content-start`, `content-end`, `content-between`, `content-around`, `content-evenly`, `content-baseline`, `content-stretch`)
- [x] Align Items (`items-start`, `items-end`, `items-center`, `items-baseline`, `items-stretch`)
- [x] Align Self (`self-auto`, `self-start`, `self-end`, `self-center`, `self-stretch`, `self-baseline`)
- [x] Place Content (`place-content-center`, `place-content-start`, `place-content-end`, `place-content-between`, `place-content-around`, `place-content-evenly`, `place-content-baseline`, `place-content-stretch`)
- [x] Place Items (`place-items-start`, `place-items-end`, `place-items-center`, `place-items-baseline`, `place-items-stretch`)
- [x] Place Self (`place-self-auto`, `place-self-start`, `place-self-end`, `place-self-center`, `place-self-stretch`)

## Spacing

- [~] Padding (`p-*`, `px-*`, `py-*`, `pt-*`, `pr-*`, `pb-*`, `pl-*`, `ps-*`, `pe-*`) — all directions done (physical + logical), negative values missing
- [~] Margin (`m-*`, `mx-*`, `my-*`, `mt-*`, `mr-*`, `mb-*`, `ml-*`, `ms-*`, `me-*`, `-m-*`) — all directions (physical + logical) + auto done, negatives missing
- [ ] Space Between (`space-x-*`, `space-y-*`, `space-x-reverse`, `space-y-reverse`)

## Sizing

- [~] Width (`w-0…96`, `w-auto`, `w-full`, `w-screen`, `w-svw`, `w-lvw`, `w-dvw`, `w-min`, `w-max`, `w-fit`, `w-1/2…11/12`) — spacing theme + keyword values done, fraction values missing
- [~] Min-Width (`min-w-0`, `min-w-full`, `min-w-min`, `min-w-max`, `min-w-fit`) — keyword statics + spacing pattern done, arbitrary values missing
- [ ] Max-Width (`max-w-xs…7xl`, `max-w-full`, `max-w-min`, `max-w-max`, `max-w-fit`, `max-w-prose`, `max-w-screen-*`, `max-w-none`)
- [x] Height (`h-0…96`, `h-auto`, `h-full`, `h-screen`, `h-svh`, `h-lvh`, `h-dvh`, `h-min`, `h-max`, `h-fit`)
- [~] Min-Height (`min-h-0`, `min-h-full`, `min-h-screen`, `min-h-svh`, `min-h-lvh`, `min-h-dvh`, `min-h-min`, `min-h-max`, `min-h-fit`) — keyword statics + spacing pattern done, arbitrary values missing
- [~] Max-Height (`max-h-0…96`, `max-h-full`, `max-h-screen`, `max-h-svh`, `max-h-lvh`, `max-h-dvh`, `max-h-min`, `max-h-max`, `max-h-fit`, `max-h-none`) — keyword statics + spacing pattern done, fraction/arbitrary values missing
- [x] Size (`size-0…96`, `size-auto`, `size-full`, `size-min`, `size-max`, `size-fit`)

## Typography

- [x] Font Family (`font-sans`, `font-serif`, `font-mono`)
- [x] Font Size (`text-xs`, `text-sm`, `text-base`, `text-lg`, `text-xl…9xl`)
- [x] Font Smoothing (`antialiased`, `subpixel-antialiased`)
- [x] Font Style (`italic`, `not-italic`)
- [x] Font Weight (`font-thin`, `font-extralight`, `font-light`, `font-normal`, `font-medium`, `font-semibold`, `font-bold`, `font-extrabold`, `font-black`)
- [x] Font Variant Numeric (`normal-nums`, `ordinal`, `slashed-zero`, `lining-nums`, `oldstyle-nums`, `proportional-nums`, `tabular-nums`, `diagonal-fractions`, `stacked-fractions`)
- [x] Letter Spacing (`tracking-tighter`, `tracking-tight`, `tracking-normal`, `tracking-wide`, `tracking-wider`, `tracking-widest`)
- [x] Line Clamp (`line-clamp-1…6`, `line-clamp-none`)
- [x] Line Height (`leading-none`, `leading-tight`, `leading-snug`, `leading-normal`, `leading-relaxed`, `leading-loose`, `leading-3…10`)
- [~] List Style Image (`list-image-none`, `list-image-[<value>]`) — keyword value done, arbitrary values missing
- [x] List Style Position (`list-inside`, `list-outside`)
- [x] List Style Type (`list-none`, `list-disc`, `list-decimal`)
- [x] Text Align (`text-left`, `text-center`, `text-right`, `text-justify`, `text-start`, `text-end`)
- [x] Text Color (`text-inherit`, `text-current`, `text-transparent`, `text-black`, `text-white`, `text-{color}-{shade}`, `text-[value]`)
- [x] Text Decoration (`underline`, `overline`, `line-through`, `no-underline`)
- [~] Text Decoration Color (`decoration-inherit`, `decoration-current`, `decoration-transparent`, `decoration-{color}-{shade}`) — keywords + theme colours done, arbitrary missing
- [x] Text Decoration Style (`decoration-solid`, `decoration-double`, `decoration-dotted`, `decoration-dashed`, `decoration-wavy`)
- [x] Text Decoration Thickness (`decoration-auto`, `decoration-from-font`, `decoration-0…8`)
- [x] Text Underline Offset (`underline-offset-auto`, `underline-offset-0…8`)
- [x] Text Transform (`uppercase`, `lowercase`, `capitalize`, `normal-case`)
- [x] Text Overflow (`truncate`, `text-ellipsis`, `text-clip`)
- [x] Text Wrap (`text-wrap`, `text-nowrap`, `text-balance`, `text-pretty`)
- [~] Text Indent (`indent-0…96`, `indent-[value]`) — spacing theme values only, arbitrary values missing
- [x] Vertical Align (`align-baseline`, `align-top`, `align-middle`, `align-bottom`, `align-text-top`, `align-text-bottom`, `align-sub`, `align-super`)
- [x] Whitespace (`whitespace-normal`, `whitespace-nowrap`, `whitespace-pre`, `whitespace-pre-line`, `whitespace-pre-wrap`, `whitespace-break-spaces`)
- [x] Word Break (`break-normal`, `break-all`, `break-keep`)
- [x] Hyphens (`hyphens-none`, `hyphens-manual`, `hyphens-auto`)
- [x] Content (`content-none`, `content-[value]`)

## Backgrounds

- [x] Background Attachment (`bg-fixed`, `bg-local`, `bg-scroll`)
- [x] Background Clip (`bg-clip-border`, `bg-clip-padding`, `bg-clip-content`, `bg-clip-text`)
- [~] Background Color (`bg-inherit`, `bg-current`, `bg-transparent`, `bg-black`, `bg-white`, `bg-{color}-{shade}`, `bg-[value]`) — keywords and theme colours done, arbitrary `bg-[value]` missing
- [x] Background Origin (`bg-origin-border`, `bg-origin-padding`, `bg-origin-content`)
- [x] Background Position (`bg-bottom`, `bg-center`, `bg-left`, `bg-right`, `bg-top`, …)
- [x] Background Repeat (`bg-repeat`, `bg-no-repeat`, `bg-repeat-x`, `bg-repeat-y`, `bg-repeat-round`, `bg-repeat-space`)
- [x] Background Size (`bg-auto`, `bg-cover`, `bg-contain`)
- [ ] Background Image (`bg-none`, `bg-gradient-to-t`, `bg-gradient-to-r`, `bg-gradient-to-b`, `bg-gradient-to-l`, …)
- [ ] Gradient Color Stops (`from-{color}`, `via-{color}`, `to-{color}`)

## Borders

- [x] Border Radius (`rounded`, `rounded-none`, `rounded-sm…3xl`, `rounded-full`, `rounded-t-*`, `rounded-r-*`, `rounded-b-*`, `rounded-l-*`, `rounded-tl-*`, …) — shorthand, directions, corners (physical + logical), edge pairs, none/full done; bare `rounded` default missing
- [x] Border Width (`border`, `border-0`, `border-2`, `border-4`, `border-8`, `border-x-*`, `border-y-*`, `border-t-*`, `border-r-*`, `border-b-*`, `border-l-*`)
- [~] Border Color (`border-inherit`, `border-current`, `border-transparent`, `border-{color}-{shade}`) — shorthand + directional (x/y/t/r/b/l/s/e) with keywords + theme done, arbitrary missing
- [x] Border Style (`border-solid`, `border-dashed`, `border-dotted`, `border-double`, `border-hidden`, `border-none`)
- [ ] Divide Width (`divide-x-*`, `divide-y-*`, `divide-x-reverse`, `divide-y-reverse`) — blocked by #23 (custom child selectors)
- [ ] Divide Color (`divide-{color}-{shade}`, `divide-[value]`) — blocked by #23 (custom child selectors)
- [ ] Divide Style (`divide-solid`, `divide-dashed`, `divide-dotted`, `divide-double`, `divide-none`) — blocked by #23 (custom child selectors)
- [x] Outline Width (`outline`, `outline-0`, `outline-1`, `outline-2`, `outline-4`, `outline-8`)
- [~] Outline Color (`outline-inherit`, `outline-current`, `outline-transparent`, `outline-{color}-{shade}`) — keywords + theme colours done, arbitrary missing
- [x] Outline Style (`outline-solid`, `outline-dashed`, `outline-dotted`, `outline-double`, `outline-none`)
- [x] Outline Offset (`outline-offset-0`, `outline-offset-1`, `outline-offset-2`, `outline-offset-4`, `outline-offset-8`)
- [ ] Ring Width (`ring`, `ring-0`, `ring-1`, `ring-2`, `ring-4`, `ring-8`, `ring-inset`)
- [ ] Ring Color (`ring-inherit`, `ring-current`, `ring-transparent`, `ring-{color}-{shade}`)
- [ ] Ring Offset Width (`ring-offset-0`, `ring-offset-1`, `ring-offset-2`, `ring-offset-4`, `ring-offset-8`)
- [ ] Ring Offset Color (`ring-offset-inherit`, `ring-offset-current`, `ring-offset-transparent`, `ring-offset-{color}-{shade}`)

## Effects

- [x] Box Shadow (`shadow`, `shadow-sm`, `shadow-md`, `shadow-lg`, `shadow-xl`, `shadow-2xl`, `shadow-inner`, `shadow-none`) — `shadow-none` + theme sizes (2xs–2xl) done, bare `shadow` default and `shadow-inner` missing
- [ ] Box Shadow Color (`shadow-inherit`, `shadow-current`, `shadow-transparent`, `shadow-{color}-{shade}`)
- [x] Opacity (`opacity-0`, `opacity-5…100`)
- [x] Mix Blend Mode (`mix-blend-normal`, `mix-blend-multiply`, `mix-blend-screen`, `mix-blend-overlay`, …)
- [x] Background Blend Mode (`bg-blend-normal`, `bg-blend-multiply`, `bg-blend-screen`, `bg-blend-overlay`, …)

## Filters

- [x] Blur (`blur`, `blur-none`, `blur-sm`, `blur-md`, `blur-lg`, `blur-xl`, `blur-2xl`, `blur-3xl`) — `blur-none` + theme sizes done, bare `blur` default missing
- [x] Brightness (`brightness-0`, `brightness-50`, `brightness-75`, `brightness-100`, `brightness-125`, `brightness-150`, `brightness-200`)
- [x] Contrast (`contrast-0`, `contrast-50`, `contrast-75`, `contrast-100`, `contrast-125`, `contrast-150`, `contrast-200`)
- [x] Drop Shadow (`drop-shadow`, `drop-shadow-sm`, `drop-shadow-md`, `drop-shadow-lg`, `drop-shadow-xl`, `drop-shadow-2xl`, `drop-shadow-none`) — `drop-shadow-none` + theme sizes done, bare `drop-shadow` default missing
- [x] Grayscale (`grayscale`, `grayscale-0`)
- [x] Hue Rotate (`hue-rotate-0`, `hue-rotate-15`, `hue-rotate-30`, `hue-rotate-60`, `hue-rotate-90`, `hue-rotate-180`)
- [x] Invert (`invert`, `invert-0`)
- [x] Saturate (`saturate-0`, `saturate-50`, `saturate-100`, `saturate-150`, `saturate-200`)
- [x] Sepia (`sepia`, `sepia-0`)
- [x] Backdrop Blur (`backdrop-blur`, `backdrop-blur-none`, `backdrop-blur-sm…3xl`)
- [x] Backdrop Brightness (`backdrop-brightness-0…200`)
- [x] Backdrop Contrast (`backdrop-contrast-0…200`)
- [x] Backdrop Grayscale (`backdrop-grayscale`, `backdrop-grayscale-0`)
- [x] Backdrop Hue Rotate (`backdrop-hue-rotate-0…180`)
- [x] Backdrop Invert (`backdrop-invert`, `backdrop-invert-0`)
- [x] Backdrop Opacity (`backdrop-opacity-0…100`)
- [x] Backdrop Saturate (`backdrop-saturate-0…200`)
- [x] Backdrop Sepia (`backdrop-sepia`, `backdrop-sepia-0`)

## Tables

- [x] Border Collapse (`border-collapse`, `border-separate`)
- [x] Border Spacing (`border-spacing-0…96`, `border-spacing-x-*`, `border-spacing-y-*`)
- [x] Table Layout (`table-auto`, `table-fixed`)
- [x] Caption Side (`caption-top`, `caption-bottom`)

## Transitions & Animation

- [x] Transition Property (`transition`, `transition-none`, `transition-all`, `transition-colors`, `transition-opacity`, `transition-shadow`, `transition-transform`)
- [x] Transition Duration (`duration-0`, `duration-75`, `duration-100`, `duration-150`, `duration-200`, `duration-300`, `duration-500`, `duration-700`, `duration-1000`)
- [x] Transition Timing Function (`ease-linear`, `ease-in`, `ease-out`, `ease-in-out`)
- [x] Transition Delay (`delay-0`, `delay-75`, `delay-100`, `delay-150`, `delay-200`, `delay-300`, `delay-500`, `delay-700`, `delay-1000`)
- [x] Transition Behavior (`transition-discrete`, `transition-normal`)
- [x] Animation (`animate-spin`, `animate-ping`, `animate-pulse`, `animate-bounce`, `animate-none`)

## Transforms

- [x] Scale (`scale-0`, `scale-50`, `scale-75`, `scale-90`, `scale-95`, `scale-100`, `scale-105`, `scale-110`, `scale-125`, `scale-150`, `scale-x-*`, `scale-y-*`)
- [x] Rotate (`rotate-0`, `rotate-1`, `rotate-2`, `rotate-3`, `rotate-6`, `rotate-12`, `rotate-45`, `rotate-90`, `rotate-180`)
- [~] Translate (`translate-x-*`, `translate-y-*`) — spacing theme values only, percentage/arbitrary values missing
- [x] Skew (`skew-x-0…12`, `skew-y-0…12`)
- [x] Transform Origin (`origin-center`, `origin-top`, `origin-top-right`, `origin-right`, `origin-bottom-right`, `origin-bottom`, `origin-bottom-left`, `origin-left`, `origin-top-left`)

## Interactivity

- [~] Accent Color (`accent-auto`, `accent-inherit`, `accent-current`, `accent-transparent`, `accent-{color}-{shade}`) — `accent-auto` + keywords + theme colours done, arbitrary missing
- [x] Appearance (`appearance-none`, `appearance-auto`)
- [x] Cursor (`cursor-auto`, `cursor-default`, `cursor-pointer`, `cursor-wait`, `cursor-text`, `cursor-move`, `cursor-help`, `cursor-not-allowed`, `cursor-none`, …)
- [~] Caret Color (`caret-inherit`, `caret-current`, `caret-transparent`, `caret-{color}-{shade}`) — keywords + theme colours done, arbitrary missing
- [x] Pointer Events (`pointer-events-none`, `pointer-events-auto`)
- [x] Resize (`resize`, `resize-none`, `resize-x`, `resize-y`)
- [x] Scroll Behavior (`scroll-auto`, `scroll-smooth`)
- [x] Scroll Margin (`scroll-m-*`, `scroll-mx-*`, `scroll-my-*`, `scroll-mt-*`, `scroll-mr-*`, `scroll-mb-*`, `scroll-ml-*`, `scroll-ms-*`, `scroll-me-*`)
- [x] Scroll Padding (`scroll-p-*`, `scroll-px-*`, `scroll-py-*`, `scroll-pt-*`, `scroll-pr-*`, `scroll-pb-*`, `scroll-pl-*`, `scroll-ps-*`, `scroll-pe-*`)
- [x] Scroll Snap Align (`snap-start`, `snap-end`, `snap-center`, `snap-align-none`)
- [x] Scroll Snap Stop (`snap-normal`, `snap-always`)
- [x] Scroll Snap Type (`snap-none`, `snap-x`, `snap-y`, `snap-both`, `snap-mandatory`, `snap-proximity`)
- [x] Touch Action (`touch-auto`, `touch-none`, `touch-pan-x`, `touch-pan-left`, `touch-pan-right`, `touch-pan-y`, `touch-pan-up`, `touch-pan-down`, `touch-pinch-zoom`, `touch-manipulation`)
- [x] User Select (`select-none`, `select-text`, `select-all`, `select-auto`)
- [x] Will Change (`will-change-auto`, `will-change-scroll`, `will-change-contents`, `will-change-transform`)
- [x] Field Sizing (`field-sizing-content`, `field-sizing-fixed`)

## SVG

- [x] Fill (`fill-none`, `fill-inherit`, `fill-current`, `fill-transparent`, `fill-{color}-{shade}`)
- [x] Stroke (`stroke-none`, `stroke-inherit`, `stroke-current`, `stroke-transparent`, `stroke-{color}-{shade}`)
- [x] Stroke Width (`stroke-0`, `stroke-1`, `stroke-2`)

## Accessibility

- [x] Screen Readers (`sr-only`, `not-sr-only`)
- [x] Forced Color Adjust (`forced-color-adjust-auto`, `forced-color-adjust-none`)
- [x] Color Scheme (`color-scheme-normal`, `color-scheme-light`, `color-scheme-dark`)
