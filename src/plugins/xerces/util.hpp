/**
 * @file
 *
 * @brief some utility functions that act as a bridge between XMLCh and std::string
 *
 * @copyright BSD License (see doc/LICENSE.md or http://www.libelektra.org)
 */

#ifndef ELEKTRA_PLUGIN_XERCES_UTIL_H
#define ELEKTRA_PLUGIN_XERCES_UTIL_H

#include <xercesc/util/XMLString.hpp>

template <typename T>
struct XercesDeleter
{
	void operator() (T * ptr) const
	{
		ptr->release ();
	}
};

template <>
struct XercesDeleter<XMLCh>
{
	void operator() (XMLCh * ptr) const
	{
		XERCES_CPP_NAMESPACE::XMLString::release (&ptr);
	}
};

template <>
struct XercesDeleter<char>
{
	void operator() (char * ptr) const
	{
		XERCES_CPP_NAMESPACE::XMLString::release (&ptr);
	}
};

template <typename T>
using xerces_unique_ptr = std::unique_ptr<T, XercesDeleter<T>>;

inline xerces_unique_ptr<XMLCh> toXMLCh (std::string const & str)
{
	return xerces_unique_ptr<XMLCh> (XERCES_CPP_NAMESPACE::XMLString::transcode (str.c_str ()));
}

inline xerces_unique_ptr<char> toCStr (XMLCh const * xmlCh)
{
	return xerces_unique_ptr<char> (XERCES_CPP_NAMESPACE::XMLString::transcode (xmlCh));
}

inline std::string toStr (XMLCh const * xmlCh)
{
	return std::string (toCStr (xmlCh).get ());
}

#define asXMLCh(str) toXMLCh (str).get ()

#endif
