// ---- start of file ----
pub mod List {

    use strum_macros::Display;
    use strum_macros::EnumString;

    use is_vowel::*;
    use rand::prelude::*;
    use rand_derive2::RandGen;

    use std::fmt;

    // ---
    #[derive(RandGen, Display)]
    pub enum NameVerb {
        Waltzing,
        Checkered,
        Lazy,
        Silver,
        Saucy,
        Flirting,
        Blue,
        Red,
        Green,
        Yellow,
        Fickle,
        Roaring,
        Carousing,
        Melting,
        Drifting,
        Spring,
        Winter,
        Summer,
        Autumn,
        Pouring,
        Heaving,
    }

    #[derive(RandGen, Display)]
    pub enum NameNoun {
        Werebear,
        Cockrel,
        Hen,
        Dragon,
        Wench,
        Dryad,
        Sky,
        Tide,
        Meadow,
        Sun,
        Fortune,
        Waters,
        Bard,
        Curmudgeon,
        Crystal,
        Mongrel,
        Ice,
        Tempest,
        Snows,
        Draft,
        Harvest,
        Chalice,
        Waves,
    }

    #[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
    pub enum MoodData {
        Jovial,
        Relaxing,
        Smoky,
        Erudite,
        Loud,
        Subdued,
        Rowdy,
        Seedy,
        Shady,
        Busy,
        LowerClass,
        MiddleClass,
        UpperClass,
        MerchantFriendly,
        EnthusiasticGamblers,
        Dour,
        Flirty,
    }

    #[derive(Display, RandGen)]
    pub enum LightingAdjectives {
        Brightly,
        Clearly,
        Evenly,
        Dimly,
        Shadowly,
    }

    #[derive(Display, RandGen)]
    pub enum LightingVerb {
        Lit,
        Illuminated,
    }

    #[derive(Display, RandGen)]
    pub enum LightingSources {
        Candles,
        AFireplace,
        OilLamps,
        MagicOrbsAndCrystals,
    }

    #[derive(Debug, Display, RandGen, Clone, Copy)]
    pub enum EstablishmentQualityLevel {
        Squalid,
        Poor,
        Modest,
        Comfortable,
        Wealthy,
        Aristocratic,
    }
    // ---
    #[derive(Debug, Display, RandGen)]
    pub enum SizeList {
        Tiny,
        Small,
        Modest,
        Large,
        Massive,
    }

    #[derive(Debug, Display)]
    pub enum BedTypeList {
        Hammocks,
        BunkBeds,
        SingleBeds,
        TentBeds,
    }

    #[derive(Display, RandGen)]
    pub enum FirstSmell {
        WoodSmoke,
        Spices,
        Perfumes,
        WearyTravellers,
        StrongDrink,
        Tobacco,
        SpicedTobacco,
        Shisha,
        FreshLinen,
        HotBread,
    }

    #[derive(Display, RandGen)]
    pub enum SecondSmell {
        FreshPastries,
        FoodsCooking,
        TheOutsideSurroundings,
        TheOcean,
        TheForests,
        FermentingWine,
        Hops,
        FermentingRye,
        HotSpicedCider,
        BakingSweets,
    }

    #[derive(Display, RandGen)]
    pub enum PostedSignLocation {
        OverTheBar,
        OnTheFrontOfTheBar,
        JustInsideTheDoor,
        JustOutsideTheDoor,
        HungFromTheFireplaceMantle,
        HungAroundTheNeckOfATrophyMountedStagsHead,
    }

    #[derive(Display, RandGen, PartialEq)]
    pub enum PostedSignMessage {
        WeDontServeAdventurers,
        WeDontServeTieflings,
        FreeMealForGoodPerformances,
        CheapRoomForGoodPerformancesPercentOff,
        WeaponsNotPermitedToBeDrawn,
        NoSpellCasting,
        WeDontServeGoblins,
        AventurersWelcomePercentOff,
        AdventurersCanEatButNoAlcoholOrRooms,
        AlcoholNotServedToHalfOrcsHalflingsOrTieflings,
        ColorfulNamesOfPriorGuests,
        WarlocksShotOnSightOnSite,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishHowCooked {
        SlowRoasted,
        SmokedAndSeasoned,
        FireRoasted,
        DryAged,
        Broiled,
        Baked,
        HoneyBraised,
        Poached,
        Fermented,
        StuffedAndBaconWrapped,
        DeepFried,
        CharcoalGroundPit,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishWhatCooked {
        MuttonLeg,
        Venison,
        PlatterFish,
        HandFish,
        WildBoarChops,
        Sausage,
        BoarRibs,
        BerryAndCheesePies,
        WolfFlank,
        Pheasant,
        SerpentSteak,
        PlainsStrider,
    }

    #[derive(RandGen, Display)]
    pub enum HouseDishWhatSide {
        RootVegtables,
        Mushrooms,
        CheeseSauce,
        HotCream,
        HardBoiledGooseEggsAndSweetDates,
        OnionSoup,
        BerrySauce,
        ChoppedPotatoes,
        MixedGreens,
        LeeksOnionsAndCatTails,
        RoastedForestNuts,
        SweetSavoryAndSpicyDippingSauces,
    }
}
// ---- end of file ----
