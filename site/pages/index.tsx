import Link from "next/link";
import { ColorschemeWithPreview } from "@/types";
import { getColorschemesWithPreview } from "@/util/config";

type HomeProps = {
  colorschemesWithPreview: ColorschemeWithPreview[];
};

export async function getStaticProps() {
  const colorschemesWithPreview = await getColorschemesWithPreview();
  return {
    props: {
      colorschemesWithPreview,
    },
  };
}

export default function Home(props: HomeProps) {
  return (
    <div className="flex flex-wrap gap-4 py-6">
      {props.colorschemesWithPreview.map((cs, i) => (
        <Link
          href={{ pathname: cs.plugin.id, query: { colorscheme: cs.name } }}
          key={i}
        >
          <div className="text-[0.25rem] hover:scale-[1.8] hover:shadow-md rounded-md overflow-hidden transition-all">
            <div dangerouslySetInnerHTML={{ __html: cs.previewHtml }} />
          </div>
          <p className="text-slate-500 text-sm mt-2 flex items-center justify-between">
            <span className="font-medium">{cs.name}</span>
            <span className="text-xs">{cs.plugin.name}</span>
          </p>
        </Link>
      ))}
    </div>
  );
}
