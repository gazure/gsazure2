use dioxus::prelude::*;
use crate::components::{PageContainer, GradientHeading, SectionHeading, GlassCard, SkillBadge, SkillVariant, CenteredContainer};

/// About page
#[component]
pub fn About() -> Element {
    rsx! {
        PageContainer {
            GradientHeading { 
                class: "text-5xl sm:text-6xl font-bold text-center mb-12",
                "About Me"
            }

            CenteredContainer {

                GlassCard { 
                    class: "text-center mb-12 p-6",
                    p { class: "text-xl text-azure-700",
                        "I'm Grant Azure. I write code sometimes and have terrible SEO"
                    }
                }

                SectionHeading { icon: "✨", text: "Skills & Technologies" }

                div { class: "grid grid-cols-2 md:grid-cols-3 gap-4 mb-12",
                    SkillBadge { text: "Rust", variant: SkillVariant::Primary }
                    SkillBadge { text: "JavaScript/TypeScript", variant: SkillVariant::Secondary }
                    SkillBadge { text: "React", variant: SkillVariant::Glass }
                    SkillBadge { text: "Dioxus", variant: SkillVariant::Primary }
                    SkillBadge { text: "PostgreSQL", variant: SkillVariant::Glass }
                    SkillBadge { text: "Docker", variant: SkillVariant::Secondary }
                }

                SectionHeading { icon: "🚀", text: "Experience" }

                GlassCard { 
                    class: "mb-12 text-center",
                    p { class: "text-lg text-azure-700",
                        "Spent 9 years making Tripit run well. Now at a startup."
                    }
                }

                SectionHeading { icon: "💌", text: "Get in Touch" }

                GlassCard { 
                    class: "text-center",
                    p { class: "text-lg text-azure-700 italic",
                        "if you know, you know."
                    }
                }
            }
        }
    }
}
