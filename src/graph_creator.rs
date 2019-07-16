use amethyst::{
    ecs::{ReadExpect, Resources, SystemData},
    renderer::{
        pass::DrawFlat2DDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
        },
        types::DefaultBackend,
        GraphCreator,
    },
    window::{ScreenDimensions, Window},
};

// This graph structure is used for creating a proper `RenderGraph` for
// rendering. A renderGraph can be thought of as the stages during a render
// pass. In our case, we are only executing one subpass (DrawFlat2D, or the
// sprite pass). This graph also needs to be rebuilt whenever the window is
// resized, so the boilerplate code for that operation is also here.
#[derive(Default)]
pub struct GameGraphCreator {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for GameGraphCreator {
    // This trait method reports to the renderer if the graph must be rebuilt,
    // usually because the window has been resized. This implementation checks
    // the screen size and returns true if it has changed.
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    // This is the core of a RenderGraph, which is building the actual graph with
    // subpasses and target images.
    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        self.dirty = false;

        // Retrieve a reference to the target window, which is created by the
        // WindowBundle
        let window = <ReadExpect<'_, Window>>::fetch(res);

        // Create a new drawing surface in our window
        let surface = factory.create_surface(&window);
        // Cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind =
            image::Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // Begin building our RenderGraph
        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            // Setting color to be same as the color of the background tile edges
            // to hide tearing between background tiles when moved for parallax effect.
            // This is a bit hackish but is the simplest solution and works for our purpose.
            Some(ClearValue::Color([0.008, 0.043, 0.067, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let sprite = graph_builder.add_node(
            SubpassBuilder::new()
                .with_group(DrawFlat2DDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let _present = graph_builder
            .add_node(PresentNode::builder(factory, surface, color).with_dependency(sprite));

        graph_builder
    }
}