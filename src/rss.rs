use xmltree::{Element, XMLNode};
use anyhow::{anyhow, Result};
use regex::Regex;

macro_rules! check_err_predicate {
    ($e: expr, $res: expr) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $res
        }
    }
}

pub fn filter_rss(rss_content: &str, filter: &Regex, title: &str) -> Result<String> {
    let mut xml = Element::parse(rss_content.as_bytes())?;
    let channel = get_element_mut(&mut xml, "channel")?;
    channel.children.retain(|child| {
        if let XMLNode::Element(item) = &child {
            if item.name != "item" {
                return true;
            }            

            let mut full_descr = check_err_predicate!(get_sub_node_text(item, "title"), true);
            full_descr.push_str("\n");
            full_descr.push_str(
                check_err_predicate!(get_sub_node_text(item, "description"), true).as_str());
            
            filter.is_match(&full_descr)
        } else {
            return true;
        }
    });

    let title_el = get_element_mut(channel, "title")?;
    title_el.children.clear();
    title_el.children.push(XMLNode::CData(title.to_owned()));

    let mut buf = Vec::new();
    xml.write(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

fn get_element<'a>(node: &'a Element, name: &str) -> Result<&'a Element> {
    node.get_child(name).ok_or(anyhow!("node not found '{}'", name))
}

fn get_element_mut<'a>(node: &'a mut Element, name: &str) -> Result<&'a mut Element> {
    node.get_mut_child(name).ok_or(anyhow!("node not found '{}'", name))
}

fn get_sub_node_text(node: &Element, sub_node: &str) -> Result<String> {
    let element = get_element(node, sub_node)?;
    let res = element.get_text().ok_or(anyhow!("{} does not have text", sub_node))?;
    Ok(res.into_owned())
}

#[cfg(test)]
mod tests {
    use xmltree::Element;

    use super::*;

    static TEST_FEED_XML: &str = include_str!("test_feed.xml");

    #[test]
    fn test_filter_rss() -> Result<()> {
        let reg = Regex::new("(Шульман.*) | (Мовчан.*)")?;
        let filtered = filter_rss(TEST_FEED_XML, &reg, "title_abc")?;
        assert!(!filtered.contains("Явлинский"));
        assert!(filtered.contains("title_abc"));
        Ok(())
    }

    #[test]
    fn test_get_element() -> Result<()> {
        let xml = Element::parse(TEST_FEED_XML.as_bytes())?;
        let channel = get_element(&xml, "channel")?;
        assert_eq!(channel.name, "channel");

        let description = get_sub_node_text(channel, "description")?;
        assert!(description.contains("Гвоздь"));
        Ok(())
    }

    #[test]
    fn test_regex() -> Result<()> {
        let reg = Regex::new("(Шульман.*) | (Мовчан.*)")?;
        assert!(reg.is_match("Статус / Екатерина Шульман* и Максим Курников"));
        Ok(())
    }
}